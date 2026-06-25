use crate::runtime::ecs::core_components::{Transform, transform};
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::math::{Float2, angle, float2};
use crate::runtime::phys::RigidBody;
use crate::runtime::phys::connector::PhysCxn;
use crate::runtime::phys::connector::phys_joint::PhysJoint;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const FIXED_DT: f32 = 1.0 / 60.0;
const SOLVER_ITERATIONS: usize = 32;
const SPRING_CO: f32 = 0.999;

pub struct ConnectorSystem {
    /// Caches: ent a, vel, ent b, inv, ang, pos, rot
    pub joint_cache: Vec<(Entity, Float2, Entity, f32, f32, Float2, f32)>,
}

impl ConnectorSystem {
    pub fn new() -> Self {
        Self {
            joint_cache: Vec::new(),
        }
    }
}

impl SystemBase for ConnectorSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        self.joint_cache.clear();

        world.for_each3_mut::<Transform, RigidBody, PhysJoint>(|entity, _transform, rb, joint| {
            for cxn in joint.cxns {
                if let Some(c) = cxn {
                    self.joint_cache.push((
                        c.cxn,
                        rb.velocity,
                        entity,
                        rb.inv_inertia,
                        rb.angular_velocity,
                        rb.position,
                        rb.rotation,
                    ));
                }
            }
        });

        for _ in 0..SOLVER_ITERATIONS {
            for jnt in self.joint_cache.iter() {
                let target_ent = jnt.0;
                let host_velocity = jnt.1;
                let host_ent = jnt.2;
                let host_inv_mass = 1.0;
                let host_inv_inertia = jnt.3;
                let host_ang_vel = jnt.4;
                let host_pos = jnt.5;
                let host_rot = jnt.6;

                let anchor_world = host_pos;

                // Modify target_success to catch both linear and angular impulses
                let target_success =
                    world.get_component_mut::<RigidBody, _>(target_ent, |target_rb| {
                        let r_host = anchor_world - host_pos;
                        let r_target = anchor_world - target_rb.position;

                        // --- Linear Constraint ---
                        let v_host_anchor = host_velocity
                            + Float2::new(-host_ang_vel * r_host.y, host_ang_vel * r_host.x);
                        let v_target_anchor = target_rb.velocity
                            + Float2::new(
                                -target_rb.angular_velocity * r_target.y,
                                target_rb.angular_velocity * r_target.x,
                            );

                        let rel_vel = v_host_anchor - v_target_anchor;
                        let inv_mass_sum = host_inv_mass + target_rb.inv_mass;

                        let total_inv_mass_x = inv_mass_sum
                            + (r_host.y * r_host.y * host_inv_inertia)
                            + (r_target.y * r_target.y * target_rb.inv_inertia);
                        let total_inv_mass_y = inv_mass_sum
                            + (r_host.x * r_host.x * host_inv_inertia)
                            + (r_target.x * r_target.x * target_rb.inv_inertia);

                        let mut impulse = Float2::new(
                            if total_inv_mass_x > 0.0 {
                                rel_vel.x / total_inv_mass_x
                            } else {
                                0.0
                            },
                            if total_inv_mass_y > 0.0 {
                                rel_vel.y / total_inv_mass_y
                            } else {
                                0.0
                            },
                        );
                        impulse *= 0.9;

                        target_rb.apply_impulse(impulse);
                        let target_torque_impulse =
                            (r_target.x * impulse.y) - (r_target.y * impulse.x);
                        target_rb.apply_angular_impulse(target_torque_impulse);

                        // --- Angular Spring/Damper Constraint ---
                        // Target relative angular velocity (0.0 means they will try to match perfectly)
                        let target_rel_ang_vel = 0.0;
                        let rel_ang_vel = host_ang_vel - target_rb.angular_velocity;
                        let ang_vel_error = rel_ang_vel - target_rel_ang_vel;

                        let inv_inertia_sum = host_inv_inertia + target_rb.inv_inertia;

                        let ang_impulse = if inv_inertia_sum > 0.0 {
                            // 0.1 acts as a spring/damping coefficient to prevent hard snapping
                            (ang_vel_error / inv_inertia_sum) * SPRING_CO
                        } else {
                            0.0
                        };

                        target_rb.apply_angular_impulse(ang_impulse);

                        // Return both impulses to apply the reaction to the host
                        (impulse, ang_impulse)
                    });

                if let Some((impulse, ang_impulse)) = target_success {
                    world.get_component_mut::<RigidBody, _>(host_ent, |host_rb| {
                        let host_impulse = -impulse;
                        let r_host = anchor_world - host_rb.position;

                        host_rb.apply_impulse(host_impulse);

                        let host_torque_impulse =
                            (r_host.x * host_impulse.y) - (r_host.y * host_impulse.x);

                        // Apply the linear anchor torque AND the equal-and-opposite angular spring impulse
                        host_rb.apply_angular_impulse(host_torque_impulse - ang_impulse);
                    });
                }
            }
        }
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
// use crate::runtime::ecs::core_components::{Transform, transform};
// use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
// use crate::runtime::math::{Float2, float2};
// use crate::runtime::phys::RigidBody;
// use crate::runtime::phys::connector::PhysCxn;
// use crate::runtime::phys::connector::phys_joint::PhysJoint;
// use std::collections::{HashMap, HashSet};
// use std::sync::Arc;

// const FIXED_DT: f32 = 1.0 / 60.0;
// const SOLVER_ITERATIONS: usize = 1;

// pub struct ConnectorSystem {
//     /// Caches: (Host Entity, Target Entity, Local Anchor Point of Host)
//     pub joint_cache: Vec<(Entity, Entity, Float2)>,
// }

// impl ConnectorSystem {
//     pub fn new() -> Self {
//         Self {
//             joint_cache: Vec::new(),
//         }
//     }
// }

// // pub fn check_if_broken(ctr: &mut PhysJoint, rb: &RigidBody) {
// //     if rb.force.length_sq() >= ctr.cxn_strength_ln_sq || rb.torque.abs() >= ctr.cxn_strength_ang {
// //         ctr.is_intact = false;
// //     }
// // }

// impl SystemBase for ConnectorSystem {
//     fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

//     fn on_update(&mut self, world: &Arc<DynamicWorld>) {
//         self.joint_cache.clear();

//         world.for_each3_mut::<Transform, RigidBody, PhysJoint>(|entity, _transform, _rb, joint| {
//             // if !joint.is_intact {
//             //     return;
//             // }

//             for w_cxn in joint.cxns.iter() {
//                 if let Some(cxn) = w_cxn {
//                     // Cache the core IDs and the anchor point needed to evaluate the constraint
//                     self.joint_cache.push((entity, cxn.cxn, cxn.anc));
//                 }
//             }
//         });

//         for _iteration in 0..SOLVER_ITERATIONS {
//             for &(host_entity, target_entity, host_local_anc) in self.joint_cache.iter() {
//                 // Track down the corresponding anchor on the target side
//                 let mut target_local_anc = None;
//                 world.get_component::<PhysJoint, _>(target_entity, |jnt| {
//                     if !jnt.is_intact {
//                         return;
//                     }
//                     for cxn in jnt.cxns.iter().flatten() {
//                         if cxn.cxn == host_entity {
//                             target_local_anc = Some(cxn.anc);
//                             break;
//                         }
//                     }
//                 });
//                 //println!("here");
//                 // Skip if the connection was broken or missing on the target side
//                 let Some(targ_local_anc) = target_local_anc else {
//                     continue;
//                 };

//                 //println!("here2");
//                 // Read Host spatial & physics properties
//                 let (mut host_pos, mut host_rot, mut host_vel, mut host_ang_vel) =
//                     (Float2::ZERO, 0.0, Float2::ZERO, 0.0);
//                 world.get_component::<Transform, _>(host_entity, |t| {
//                     host_pos = t.position;
//                     host_rot = t.rotation;
//                 });
//                 world.get_component::<RigidBody, _>(host_entity, |rb| {
//                     host_vel = rb.velocity;
//                     host_ang_vel = rb.angular_velocity;
//                 });

//                 // Read Target spatial & physics properties
//                 let (mut targ_pos, mut targ_rot, mut targ_vel, mut targ_ang_vel) =
//                     (Float2::ZERO, 0.0, Float2::ZERO, 0.0);
//                 world.get_component::<Transform, _>(target_entity, |t| {
//                     targ_pos = t.position;
//                     targ_rot = t.rotation;
//                 });
//                 world.get_component::<RigidBody, _>(target_entity, |rb| {
//                     targ_vel = rb.velocity;
//                     targ_ang_vel = rb.angular_velocity;
//                 });

//                 // Establish bolt locations for both bodies in world space
//                 let (host_bolt_a, host_bolt_b) = get_bolts(host_pos, host_local_anc, host_rot);
//                 let (targ_bolt_a, targ_bolt_b) = get_bolts(targ_pos, targ_local_anc, targ_rot);

//                 // Compute exact velocities at specific bolt points (Linear + Angular Cross Product)
//                 let host_r_a = host_bolt_a - host_pos;
//                 let host_vel_a = host_vel + Float2::new(-host_r_a.y, host_r_a.x) * host_ang_vel;

//                 let host_r_b = host_bolt_b - host_pos;
//                 let host_vel_b = host_vel + Float2::new(-host_r_b.y, host_r_b.x) * host_ang_vel;

//                 let targ_r_a = targ_bolt_a - targ_pos;
//                 let targ_vel_a = targ_vel + Float2::new(-targ_r_a.y, targ_r_a.x) * targ_ang_vel;

//                 let targ_r_b = targ_bolt_b - targ_pos;
//                 let targ_vel_b = targ_vel + Float2::new(-targ_r_b.y, targ_r_b.x) * targ_ang_vel;

//                 // Calculate individual spring forces
//                 let fa = calc_spring_force(host_bolt_a, targ_bolt_a, host_vel_a, targ_vel_a);
//                 let fb = calc_spring_force(host_bolt_b, targ_bolt_b, host_vel_b, targ_vel_b);

//                 // 3. APPLY FORCES
//                 // Accumulate to target body
//                 world.get_component_mut::<RigidBody, _>(target_entity, |rb| {
//                     if let Some(f) = fa {
//                         rb.apply_force_at(f, targ_bolt_a);
//                     }
//                     if let Some(f) = fb {
//                         rb.apply_force_at(f, targ_bolt_b);
//                     }
//                 });

//                 // Apply inverse force to host body (Newton's Third Law)
//                 world.get_component_mut::<RigidBody, _>(host_entity, |rb| {
//                     if let Some(f) = fa {
//                         rb.apply_force_at(-f, host_bolt_a);
//                     }
//                     if let Some(f) = fb {
//                         rb.apply_force_at(-f, host_bolt_b);
//                     }
//                 });
//             }
//         }

//         // // 4. POST-SOLVER DESTRUCTION CHECK
//         // // Check if the accumulated solver forces broke the structural limits
//         // world.for_each2_mut::<RigidBody, PhysJoint>(|_, rb, joint| {
//         //     if joint.is_intact {
//         //         check_if_broken(joint, rb);
//         //     }
//         // });
//     }

//     fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
// }

// const SPRING_STIFFNESS: f32 = 1       .0;
// const SPRING_DAMPING: f32 = 1.0;

// /// World pos for two bolts aligned vertically along the joint interface
// fn get_bolts(pos: Float2, anc: Float2, rot: f32) -> (Float2, Float2) {
//     // Instead of using the anchor's direction, always use a fixed vertical split (Y-axis)
//     // This ensures Bolt A is always "Top" and Bolt B is always "Bottom" for every joint interface
//     let vertical_offset = Float2::new(0.0, 0.5);

//     let bolt_a = pos + (anc + vertical_offset).rotate(rot);
//     let bolt_b = pos + (anc - vertical_offset).rotate(rot);

//     (bolt_a, bolt_b)
// }

// fn calc_spring_force(
//     host_anc_pos: Float2,
//     target_anc_pos: Float2,
//     host_vel: Float2,
//     target_vel: Float2,
// ) -> Option<Float2> {
//     let delta = host_anc_pos - target_anc_pos;
//     let current_length = delta.length();

//     println!("delta {}", delta);
//     if current_length < 0.001 {
//         return None;
//     }

//     let force_dir = delta / current_length;
//     let spring_force_mag = SPRING_STIFFNESS * current_length;

//     let relative_velocity = host_vel - target_vel;
//     let damping_force_mag = relative_velocity.dot(force_dir) * SPRING_DAMPING;

//     let total_force = spring_force_mag - damping_force_mag;
//     Some(force_dir * total_force)
// }
// self.joint_cache.clear();

// world.for_each3_mut::<Transform, RigidBody, PhysJoint>(|entity, transform, rb, joint| {
//     for w_cxn in joint.cxns.iter() {
//         if let Some(cxn) = w_cxn {
//             // Calculate anchor position in world space for the host entity
//             let host_anc_w_pos = transform.position + (cxn.anc.rotate(transform.rotation));

//             // Cache: (Host Entity, Target Entity, Host Anchor World Pos)
//             self.joint_cache.push((entity, cxn.cxn, host_anc_w_pos));
//         }
//     }
// });

// // Tuning constants
// const SPRING_STIFFNESS: f32 = 100.0;
// const SPRING_DAMPING: f32 = 1.0; // Keeps things from vibrating endlessly
// const TORSIONAL_MULT: f32 = 0.01;
// const ANGULAR_DAMP: f32 = 0.1;

// for &(host_entity, target_entity, host_anc_pos) in self.joint_cache.iter() {
//     let mut target_pos = Float2::ZERO;
//     let mut target_vel = Float2::ZERO;
//     let mut host_vel = Float2::ZERO;

//     let mut target_rot = 0.0;
//     let mut host_rot = 0.0;
//     let mut host_ang_vel = 0.0;
//     let mut target_ang_vel = 0.0;
//     // Gather target data
//     world.get_component::<Transform, _>(target_entity, |t| {
//         target_pos = t.position;
//         target_rot = t.rotation;
//     });
//     world.get_component::<RigidBody, _>(target_entity, |rb| {
//         target_vel = rb.velocity;
//         target_ang_vel = rb.angular_velocity;
//     });

//     let mut targ_ancs: [Option<PhysCxn>; 4] = [None; 4];
//     world.get_component::<PhysJoint, _>(target_entity, |jnt| {
//         targ_ancs = jnt.cxns;
//     });

//     world.get_component::<Transform, _>(host_entity, |t| {
//         host_rot = t.rotation;
//     });

//     world.get_component::<RigidBody, _>(host_entity, |rb| {
//         host_vel = rb.velocity;
//         host_ang_vel = rb.angular_velocity;
//     });

//     let delta = host_anc_pos - target_pos;
//     let current_length = delta.length();

//     if current_length < 0.001 {
//         continue;
//     }

//     let force_dir = delta / current_length;

//     let spring_force_mag = SPRING_STIFFNESS * (current_length);

//     let relative_velocity = host_vel - target_vel;
//     let damping_force_mag = relative_velocity.dot(force_dir) * SPRING_DAMPING;

//     // Total force scalar
//     let total_force = spring_force_mag - damping_force_mag;

//     // Fix later if needed
//     let force_imp = force_dir * total_force;

//     // let simp_torque = -(target_rot - host_rot) * TORSIONAL_MULT;
//     // let damper = ANGULAR_DAMP * (host_ang_vel - target_ang_vel).abs();
//     // let damp_torque = simp_torque / damper;
//     //
//     world.get_component_mut::<RigidBody, _>(target_entity, |rb| {
//         rb.apply_force_at(force_imp, host_anc_pos);
//         //rb.apply_impulse(force_imp, Float2::ZERO);
//     });

//     world.get_component_mut::<RigidBody, _>(host_entity, |rb| {
//         rb.apply_force_at(-force_imp, host_anc_pos);
//         //rb.apply_impulse(-force_imp, Float2::ZERO);
//     });
// use crate::runtime::ecs::core_components::{Transform, transform};
// use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
// use crate::runtime::math::Float2;
// use crate::runtime::phys::RigidBody;
// use crate::runtime::phys::connector::PhysCxn;
// use crate::runtime::phys::connector::phys_joint::PhysJoint;
// use std::collections::{HashMap, HashSet};
// use std::sync::Arc;

// const FIXED_DT: f32 = 1.0 / 60.0;
// const SOLVER_ITERATIONS: usize = 4;

// pub struct ConnectorSystem {
//     /// For lookups in the solve phase entity and then world pos to move to
//     pub joint_cache: Vec<(Entity, Entity, Float2)>,
// }

// impl ConnectorSystem {
//     pub fn new() -> Self {
//         Self {
//             joint_cache: Vec::new(),
//         }
//     }
// }

// pub fn check_if_broken(ctr: &mut PhysJoint, rb: &RigidBody) {
//     if rb.force.length_sq() >= ctr.cxn_strength_ln_sq || rb.torque.abs() >= ctr.cxn_strength_ang {
//         ctr.is_intact = false;
//     }
// }

// impl SystemBase for ConnectorSystem {
//     fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

//     fn on_update(&mut self, world: &Arc<DynamicWorld>) {
//         self.joint_cache.clear();

//         world.for_each3_mut::<Transform, RigidBody, PhysJoint>(|entity, transform, rb, joint| {
//             for w_cxn in joint.cxns.iter() {
//                 if let Some(cxn) = w_cxn {
//                     // Calculate anchor position in world space for the host entity
//                     let host_anc_w_pos = transform.position + (cxn.anc.rotate(transform.rotation));

//                     // Cache: (Host Entity, Target Entity, Host Anchor World Pos)
//                     self.joint_cache.push((entity, cxn.cxn, host_anc_w_pos));
//                 }
//             }
//         });

//         // Tuning constants
//         const SPRING_STIFFNESS: f32 = 100.0;
//         const SPRING_DAMPING: f32 = 5.0; // Keeps things from vibrating endlessly
//         const TORQUE_STIFFNESS: f32 = 0.1; // Higher = stiffer, resists bending more
//         const TORQUE_DAMPING: f32 = 0.0; // Keeps it from wobbling wildly
//         const REST_ANGLE_DIFF: f32 = 0.0;

//         for &(host_entity, target_entity, host_anc_pos) in self.joint_cache.iter() {
//             let mut target_pos = Float2::ZERO;
//             let mut target_vel = Float2::ZERO;
//             let mut host_vel = Float2::ZERO;

//             let mut host_rot = 0.0;
//             let mut target_rot = 0.0;
//             let mut host_ang_vel = 0.0;
//             let mut target_ang_vel = 0.0;

//             // Gather target data
//             world.get_component::<Transform, _>(target_entity, |t| {
//                 target_pos = t.position;
//                 target_rot = t.rotation;
//             });
//             world.get_component::<RigidBody, _>(target_entity, |rb| {
//                 target_vel = rb.velocity;
//                 target_ang_vel = rb.angular_velocity;
//             });

//             world.get_component::<Transform, _>(host_entity, |t| host_rot = t.rotation);
//             // Gather host velocity for damping
//             world.get_component::<RigidBody, _>(host_entity, |rb| {
//                 host_vel = rb.velocity;
//                 host_ang_vel = rb.angular_velocity;
//             });

//             // Calculate delta vector (from target to host anchor)
//             let delta = host_anc_pos - target_pos;
//             let current_length = delta.length();

//             // Guard against division by zero (NaN trap)
//             if current_length < 0.001 {
//                 continue;
//             }

//             let force_dir = delta / current_length;

//             // --- Physics Calculations ---
//             // Hooke's Law: F = -k * (x - x0)
//             let spring_force_mag = SPRING_STIFFNESS * (current_length);

//             // Damping: Damping force opposes relative velocity along the spring axis
//             let relative_velocity = host_vel - target_vel;
//             let damping_force_mag = relative_velocity.dot(force_dir) * SPRING_DAMPING;

//             // Total force scalar
//             let total_force = (spring_force_mag + damping_force_mag);

//             // Fix later if needed
//             let force_imp = force_dir * total_force * 0.5;

//             // Ang pass
//             let mut angle_diff = host_rot - target_rot;
//             println!("angle diff {}", angle_diff);

//             // Keep angle_diff normalized between -PI and PI so it takes the shortest rotation path
//             angle_diff = (angle_diff + std::f32::consts::PI).rem_euclid(2.0 * std::f32::consts::PI)
//                 - std::f32::consts::PI;

//             // 3. Angular Hooke's Law
//             let torque_spring = TORQUE_STIFFNESS * (angle_diff - REST_ANGLE_DIFF);

//             // 4. Angular Damping
//             let relative_ang_vel = host_ang_vel - target_ang_vel;
//             let torque_damping = TORQUE_DAMPING * relative_ang_vel;

//             // Total torque to apply
//             let total_torque = torque_spring;

//             world.get_component_mut::<RigidBody, _>(target_entity, |rb| {
//                 rb.apply_force(force_imp);
//                 rb.apply_torque(-total_torque);
//             });

//             // Pull host entity in the exact opposite direction
//             world.get_component_mut::<RigidBody, _>(host_entity, |rb| {
//                 rb.apply_force(-force_imp);
//                 //rb.apply_torque(-total_torque);
//             });
//         }
//     }

//     fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
// }

// use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
// use crate::runtime::math::Float2;
// use crate::runtime::phys::RigidBody;
// use crate::runtime::phys::connector::PhysCxn;
// use crate::runtime::phys::connector::phys_joint::PhysJoint;
// use std::collections::{HashMap, HashSet};
// use std::sync::Arc;

// const FIXED_DT: f32 = 1.0 / 60.0;
// const SOLVER_ITERATIONS: usize = 4;

// pub struct ConnectorSystem {
//     /// Cxn and dynamic constraints cached for iterative resolution
//     pub joint_cache: Vec<(Entity, PhysCxn)>,
// }

// impl ConnectorSystem {
//     pub fn new() -> Self {
//         Self {
//             joint_cache: Vec::new(),
//         }
//     }
// }

// pub fn check_if_broken(ctr: &mut PhysJoint, rb: &RigidBody) {
//     if rb.force.length_sq() >= ctr.cxn_strength_ln_sq || rb.torque.abs() >= ctr.cxn_strength_ang {
//         ctr.is_intact = false;
//     }
// }

// impl SystemBase for ConnectorSystem {
//     fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

//     fn on_update(&mut self, world: &Arc<DynamicWorld>) {
//         self.joint_cache.clear();

//         // --- PASS 1: Check Breaks & Collect Live Joints ---
//         let mut broken_entities = HashSet::new();

//         world.for_each2_mut_both::<RigidBody, PhysJoint>(|entity, rb, joint| {
//             check_if_broken(joint, rb);
//             if !joint.is_intact {
//                 broken_entities.insert(entity);
//                 return;
//             }

//             for cxn in joint.cxns.iter() {
//                 if let Some(u_cxn) = cxn {
//                     self.joint_cache.push((entity, *u_cxn));
//                 }
//             }
//         });

//         // Break references if their connected counterparts snapped
//         if !broken_entities.is_empty() {
//             world.for_each_mut::<PhysJoint>(|_, joint| {
//                 for cxn in joint.cxns.iter_mut() {
//                     if let Some(u_cxn) = cxn {
//                         if broken_entities.contains(&u_cxn.cxn) {
//                             *cxn = None;
//                         }
//                     }
//                 }
//             });
//             // Prune cached entries pointing to dead connections
//             self.joint_cache.retain(|(entity, u_cxn)| {
//                 !broken_entities.contains(entity) && !broken_entities.contains(&u_cxn.cxn)
//             });
//         }

//         // --- PASS 2: Collect Body Physics References for Fast Iteration ---
//         let mut unique_entities = HashSet::new();
//         for (e_a, u_cxn) in &self.joint_cache {
//             unique_entities.insert(*e_a);
//             unique_entities.insert(u_cxn.cxn);
//         }

//         let mut bodies: HashMap<Entity, RigidBody> = HashMap::new();
//         for entity in unique_entities {
//             world.get_component::<RigidBody, _>(entity, |rb| {
//                 bodies.insert(entity, rb.clone());
//             });
//         }

//         // --- PASS 3: Iterative Constraint Solver ---
//         // Get a raw pointer to the map to cleanly bypass the double-mutable borrow restriction
//         let bodies_ptr = &mut bodies as *mut HashMap<Entity, RigidBody>;

//         for _ in 0..SOLVER_ITERATIONS {
//             for &(entity_a, cxn) in &self.joint_cache {
//                 let entity_b = cxn.cxn;

//                 if entity_a == entity_b {
//                     continue;
//                 }

//                 // Safely extract separate mutable references from the raw map pointer
//                 let (rb_a, rb_b) = unsafe {
//                     match (
//                         (*bodies_ptr).get_mut(&entity_a),
//                         (*bodies_ptr).get_mut(&entity_b),
//                     ) {
//                         (Some(a), Some(b)) => (a, b),
//                         _ => continue,
//                     }
//                 };

//                 let inv_m_a = rb_a.inv_mass;
//                 let inv_m_b = rb_b.inv_mass;
//                 let inv_i_a = rb_a.inv_inertia;
//                 let inv_i_b = rb_b.inv_inertia;

//                 // 1. Angular Velocity Constraint Resolution (Weld matching speeds)
//                 let relative_angular_vel = rb_b.angular_velocity - rb_a.angular_velocity;
//                 let angular_mass = inv_i_a + inv_i_b;
//                 if angular_mass > 0.0 {
//                     let angular_impulse = relative_angular_vel / angular_mass;
//                     rb_a.angular_velocity += angular_impulse * inv_i_a;
//                     rb_b.angular_velocity -= angular_impulse * inv_i_b;
//                 }

//                 // 2. Linear Velocity Constraint Resolution at Anchors
//                 let r_a = cxn.anc.rotate(rb_a.rotation);
//                 let r_b = cxn.anc.rotate(rb_b.rotation);

//                 // Compute relative velocity at the exact anchor point contact surface
//                 let v_anchor_a = rb_a.velocity
//                     + Float2::new(
//                         -r_a.y * rb_a.angular_velocity,
//                         r_a.x * rb_a.angular_velocity,
//                     );
//                 let v_anchor_b = rb_b.velocity
//                     + Float2::new(
//                         -r_b.y * rb_b.angular_velocity,
//                         r_b.x * rb_b.angular_velocity,
//                     );
//                 let relative_linear_vel = v_anchor_b - v_anchor_a;

//                 // Calculate the effective linear mass matrix components for this joint
//                 let k_matrix_x =
//                     inv_m_a + inv_m_b + r_a.y * r_a.y * inv_i_a + r_b.y * r_b.y * inv_i_b;
//                 let k_matrix_y =
//                     inv_m_a + inv_m_b + r_a.x * r_a.x * inv_i_a + r_b.x * r_b.x * inv_i_b;

//                 if k_matrix_x > 0.0 && k_matrix_y > 0.0 {
//                     let linear_impulse = Float2::new(
//                         relative_linear_vel.x / k_matrix_x,
//                         relative_linear_vel.y / k_matrix_y,
//                     );

//                     // Apply linear velocity changes
//                     rb_a.velocity += linear_impulse * inv_m_a;
//                     rb_b.velocity -= linear_impulse * inv_m_b;

//                     // Apply resulting anchor torque adjustments
//                     rb_a.angular_velocity += r_a.cross(linear_impulse) * inv_i_a;
//                     rb_b.angular_velocity -= r_b.cross(linear_impulse) * inv_i_b;
//                 }
//             }
//         }

//         // --- PASS 4: Flush Solved Velocities Back to World Components ---
//         for (entity, solved_rb) in bodies {
//             world.get_component_mut::<RigidBody, _>(entity, |rb| {
//                 rb.velocity = solved_rb.velocity;
//                 rb.angular_velocity = solved_rb.angular_velocity;
//             });
//         }
//     }

//     fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
// }
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
