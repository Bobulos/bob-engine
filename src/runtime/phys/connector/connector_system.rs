use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::math::Float2;
use crate::runtime::phys::RigidBody;
use crate::runtime::phys::connector::PhysCxn;
use crate::runtime::phys::connector::phys_joint::PhysJoint;

const FIXED_DT: f32 = 1.0 / 60.0;
const SOLVER_ITERATIONS: usize = 1;

pub struct ConnectorSystem {
    /// Cxn and dynamic constraints cached for iterative resolution
    pub joint_cache: Vec<(Entity, PhysCxn)>,
}

impl ConnectorSystem {
    pub fn new() -> Self {
        Self {
            joint_cache: Vec::new(),
        }
    }
}

pub fn check_if_broken(ctr: &mut PhysJoint, rb: &RigidBody) {
    if rb.force.length_sq() >= ctr.cxn_strength_ln_sq || rb.torque.abs() >= ctr.cxn_strength_ang {
        ctr.is_intact = false;
    }
}

impl SystemBase for ConnectorSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        self.joint_cache.clear();

        // --- PASS 1: Check Breaks & Collect Live Joints ---
        let mut broken_entities = HashSet::new();

        world.for_each2_mut_both::<RigidBody, PhysJoint>(|entity, rb, joint| {
            check_if_broken(joint, rb);
            if !joint.is_intact {
                broken_entities.insert(entity);
                return;
            }

            for cxn in joint.cxns.iter() {
                if let Some(u_cxn) = cxn {
                    self.joint_cache.push((entity, *u_cxn));
                }
            }
        });

        // Break references if their connected counterparts snapped
        if !broken_entities.is_empty() {
            world.for_each_mut::<PhysJoint>(|_, joint| {
                for cxn in joint.cxns.iter_mut() {
                    if let Some(u_cxn) = cxn {
                        if broken_entities.contains(&u_cxn.cxn) {
                            *cxn = None;
                        }
                    }
                }
            });
            // Prune cached entries pointing to dead connections
            self.joint_cache.retain(|(entity, u_cxn)| {
                !broken_entities.contains(entity) && !broken_entities.contains(&u_cxn.cxn)
            });
        }

        // --- PASS 2: Collect Body Physics References for Fast Iteration ---
        // Grab unique list of all entities involved in the joints to pull their bodies out safely
        let mut unique_entities = HashSet::new();
        for (e_a, u_cxn) in &self.joint_cache {
            unique_entities.insert(*e_a);
            unique_entities.insert(u_cxn.cxn);
        }

        // Pull positions/velocities cleanly without holding borrow logs across iterations
        // We'll update the world components *after* the iterations finish.
        let mut bodies: HashMap<Entity, RigidBody> = HashMap::new();
        for entity in unique_entities {
            world.get_component::<RigidBody, _>(entity, |rb| {
                bodies.insert(entity, rb.clone());
            });
        }

        // --- PASS 3: Iterative Constraint Solver ---
        for _ in 0..SOLVER_ITERATIONS {
            for &(entity_a, cxn) in &self.joint_cache {
                let entity_b = cxn.cxn;

                // Split borrow safely out of our local map
                // let (rb_a, rb_b) = match (bodies.get_mut(&entity_a), bodies.get_mut(&entity_b)) {
                //     (Some(a), Some(b)) => (a, b),
                //     _ => continue,
                // };
                if entity_a == entity_b {
                    continue;
                }

                // 2. Get raw mutable pointers to bypass the double-borrow restriction
                let rb_a_ptr = bodies.get_mut(&entity_a).map(|rb| rb as *mut RigidBody);
                let rb_b_ptr = bodies.get_mut(&entity_b).map(|rb| rb as *mut RigidBody);

                // 3. Safely dereference them inside an unsafe block
                let (rb_a, rb_b) = match (rb_a_ptr, rb_b_ptr) {
                    (Some(a_ptr), Some(b_ptr)) => unsafe { (&mut *a_ptr, &mut *b_ptr) },
                    _ => continue,
                };
                // Assume rigid bodies expose standard mass/inertia properties.
                // If your RigidBody is static/kinematic, inv_mass = 0.0
                let inv_m_a = rb_a.inv_mass; // Replace with rb_a.inv_mass if available
                let inv_m_b = rb_b.inv_mass;
                let inv_i_a = rb_a.inv_inertia; // Replace with rb_a.inv_inertia if available
                let inv_i_b = rb_b.inv_inertia;

                // 1. Angular Velocity Constraint Resolution (Weld matching speeds)
                let relative_angular_vel = rb_b.angular_velocity - rb_a.angular_velocity;
                let angular_mass = inv_i_a + inv_i_b;
                if angular_mass > 0.0 {
                    let angular_impulse = relative_angular_vel / angular_mass;
                    rb_a.angular_velocity += angular_impulse * inv_i_a;
                    rb_b.angular_velocity -= angular_impulse * inv_i_b;
                }

                // 2. Linear Velocity Constraint Resolution at Anchors
                // Convert your local anchors into world space offsets relative to mass center
                let r_a = cxn.anc.rotate(rb_a.rotation); // Assuming Float2 has a .rotate() method
                let r_b = cxn.anc.rotate(rb_b.rotation);

                // Compute relative velocity at the exact anchor point contact surface
                let v_anchor_a = rb_a.velocity
                    + Float2::new(
                        -r_a.y * rb_a.angular_velocity,
                        r_a.x * rb_a.angular_velocity,
                    );
                let v_anchor_b = rb_b.velocity
                    + Float2::new(
                        -r_b.y * rb_b.angular_velocity,
                        r_b.x * rb_b.angular_velocity,
                    );
                let relative_linear_vel = v_anchor_b - v_anchor_a;

                // Calculate the effective linear mass matrix components for this joint
                // K = (M_a^-1 + M_b^-1) * I - r_a_skew^2 * I_a^-1 - r_b_skew^2 * I_b^-1
                let k_matrix_x =
                    inv_m_a + inv_m_b + r_a.y * r_a.y * inv_i_a + r_b.y * r_b.y * inv_i_b;
                let k_matrix_y =
                    inv_m_a + inv_m_b + r_a.x * r_a.x * inv_i_a + r_b.x * r_b.x * inv_i_b;

                if k_matrix_x > 0.0 && k_matrix_y > 0.0 {
                    let linear_impulse = Float2::new(
                        relative_linear_vel.x / k_matrix_x,
                        relative_linear_vel.y / k_matrix_y,
                    );

                    // Apply linear velocity changes
                    rb_a.velocity += linear_impulse * inv_m_a;
                    rb_b.velocity -= linear_impulse * inv_m_b;

                    // Apply resulting anchor torque adjustments
                    rb_a.angular_velocity += r_a.cross(linear_impulse) * inv_i_a;
                    rb_b.angular_velocity -= r_b.cross(linear_impulse) * inv_i_b;
                }
            }
        }

        // --- PASS 4: Flush Solved Velocities Back to World Components ---
        for (entity, solved_rb) in bodies {
            world.get_component_mut::<RigidBody, _>(entity, |rb| {
                rb.velocity = solved_rb.velocity;
                rb.angular_velocity = solved_rb.angular_velocity;
            });
        }
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}

// use std::collections::{HashMap, HashSet};
// use std::sync::Arc;

// use crate::runtime::ecs::{DynamicWorld, SystemBase};
// use crate::runtime::math::Float2;
// use crate::runtime::phys::RigidBody;
// use crate::runtime::phys::connector::physics_connector::PhysicsConnector;

// const FIXED_DT: f32 = 1.0 / 60.0;
// const SOLVER_ITERATIONS: usize = 12;

// pub struct ConnectorSystem;

// impl ConnectorSystem {
//     pub fn new() -> Self {
//         Self
//     }
// }

// pub fn check_if_broken(connector: &mut PhysicsConnector, rb: &RigidBody) {
//     if rb.force.length_sq() >= connector.connection_strength_ln_sq
//         || rb.torque >= connector.connection_strength_ang
//     {
//         connector.is_intact = false;
//     }
// }

// /// Solve a single constraint immediately (Gauss-Seidel step)
// fn solve_constraint(
//     rb_a: &RigidBody,
//     rb_b: &RigidBody,
//     anchor_a_local: Float2,
//     anchor_b_local: Float2,
// ) -> (Float2, Float2, f32) {
//     let beta = 0.15;

//     let (sin_a, cos_a) = rb_a.rotation.sin_cos();
//     let r_a = Float2::new(
//         anchor_a_local.x * cos_a - anchor_a_local.y * sin_a,
//         anchor_a_local.x * sin_a + anchor_a_local.y * cos_a,
//     );

//     let (sin_b, cos_b) = rb_b.rotation.sin_cos();
//     let r_b = Float2::new(
//         anchor_b_local.x * cos_b - anchor_b_local.y * sin_b,
//         anchor_b_local.x * sin_b + anchor_b_local.y * cos_b,
//     );

//     let world_a = rb_a.position + r_a;
//     let world_b = rb_b.position + r_b;

//     let positional_error = world_b - world_a;

//     let v_a = rb_a.velocity_at(r_a);
//     let v_b = rb_b.velocity_at(r_b);
//     let relative_v = v_b - v_a;

//     let linear_bias = -positional_error * (beta / FIXED_DT);

//     let inv_mass_sum = rb_a.inv_mass + rb_b.inv_mass;

//     let k_x =
//         inv_mass_sum + (r_a.y * r_a.y * rb_a.inv_inertia) + (r_b.y * r_b.y * rb_b.inv_inertia);

//     let k_y =
//         inv_mass_sum + (r_a.x * r_a.x * rb_a.inv_inertia) + (r_b.x * r_b.x * rb_b.inv_inertia);

//     let j_x = (-relative_v.x + linear_bias.x) / k_x.max(1e-6);
//     let j_y = (-relative_v.y + linear_bias.y) / k_y.max(1e-6);

//     let impulse = Float2::new(j_x, j_y);

//     // angular correction
//     let angular_error = (rb_b.rotation - rb_a.rotation + std::f32::consts::PI)
//         .rem_euclid(2.0 * std::f32::consts::PI)
//         - std::f32::consts::PI;

//     let relative_w = rb_b.angular_velocity - rb_a.angular_velocity;

//     let angular_bias = (-angular_error * beta / FIXED_DT).clamp(-1.0, 1.0);

//     let k_ang = rb_a.inv_inertia + rb_b.inv_inertia;

//     let j_ang = if k_ang > 0.0 {
//         (-relative_w + angular_bias) / k_ang
//     } else {
//         0.0
//     };

//     (impulse, r_a, j_ang)
// }

// impl SystemBase for ConnectorSystem {
//     fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

//     fn on_update(&mut self, world: &Arc<DynamicWorld>) {
//         if FIXED_DT <= 0.0 {
//             return;
//         }

//         // -----------------------------
//         // 1. Break detection
//         // -----------------------------
//         world.for_each2_mut_both::<PhysicsConnector, RigidBody>(|_, connector, rb| {
//             check_if_broken(connector, rb);
//         });

//         let mut broken = HashSet::new();

//         world.for_each::<PhysicsConnector>(|e, c| {
//             if !c.is_intact {
//                 broken.insert(e);
//             }
//         });

//         world.for_each_mut::<PhysicsConnector>(|_, connector| {
//             for (i, c) in connector.connections.iter_mut().enumerate() {
//                 if let Some(other) = c {
//                     if broken.contains(other) {
//                         *c = None;
//                         connector.anchors[i] = None;
//                     }
//                 }
//             }
//         });

//         // -----------------------------
//         // 2. Build working body cache
//         // -----------------------------
//         let mut bodies: HashMap<_, RigidBody> = HashMap::new();
//         world.for_each::<RigidBody>(|e, rb| {
//             bodies.insert(e, rb.clone());
//         });

//         // -----------------------------
//         // 3. Iterative solver (Gauss-Seidel)
//         // -----------------------------
//         for _ in 0..SOLVER_ITERATIONS {
//             let mut updates_linear: HashMap<_, Float2> = HashMap::new();
//             let mut updates_angular: HashMap<_, f32> = HashMap::new();

//             world.for_each::<PhysicsConnector>(|entity, connector| {
//                 if !connector.is_intact {
//                     return;
//                 }

//                 let Some(rb_a) = bodies.get(&entity).cloned() else {
//                     return;
//                 };

//                 for (i, &maybe_b) in connector.connections.iter().enumerate() {
//                     let Some(b) = maybe_b else {
//                         continue;
//                     };
//                     let Some(rb_b) = bodies.get(&b).cloned() else {
//                         continue;
//                     };
//                     let Some(anchor_a) = connector.anchors[i] else {
//                         continue;
//                     };

//                     let anchor_b = Float2::ZERO;

//                     let (impulse, r_a, ang) = solve_constraint(&rb_a, &rb_b, anchor_a, anchor_b);

//                     updates_linear
//                         .entry(entity)
//                         .and_modify(|v| *v -= impulse)
//                         .or_insert(-impulse);

//                     updates_linear
//                         .entry(b)
//                         .and_modify(|v| *v += impulse)
//                         .or_insert(impulse);

//                     if ang != 0.0 {
//                         *updates_angular.entry(entity).or_insert(0.0) -= ang;
//                         *updates_angular.entry(b).or_insert(0.0) += ang;
//                     }
//                 }
//             });

//             // apply immediately into working bodies (THIS is the key fix)
//             for (e, imp) in updates_linear.drain() {
//                 if let Some(rb) = bodies.get_mut(&e) {
//                     rb.apply_impulse(imp, Float2::ZERO);
//                 }
//             }

//             for (e, j) in updates_angular.drain() {
//                 if let Some(rb) = bodies.get_mut(&e) {
//                     rb.angular_velocity += j * rb.inv_inertia;
//                 }
//             }
//         }

//         // -----------------------------
//         // 4. Write back to ECS
//         // -----------------------------
//         for (e, rb) in bodies {
//             world.get_component_mut::<RigidBody, _>(e, |dst| {
//                 *dst = rb;
//             });
//         }
//     }

//     fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
// }
