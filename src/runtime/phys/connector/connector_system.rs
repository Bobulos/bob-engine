use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::Float2;
use crate::runtime::phys::RigidBody;
use crate::runtime::phys::connector::physics_connector::PhysicsConnector;

const FIXED_DT: f32 = 1.0 / 60.0;
const SOLVER_ITERATIONS: usize = 12;

pub struct ConnectorSystem;

impl ConnectorSystem {
    pub fn new() -> Self {
        Self
    }
}

pub fn check_if_broken(connector: &mut PhysicsConnector, rb: &RigidBody) {
    if rb.force.length_sq() >= connector.connection_strength_ln_sq
        || rb.torque >= connector.connection_strength_ang
    {
        connector.is_intact = false;
    }
}

/// Solve a single constraint immediately (Gauss-Seidel step)
fn solve_constraint(
    rb_a: &RigidBody,
    rb_b: &RigidBody,
    anchor_a_local: Float2,
    anchor_b_local: Float2,
) -> (Float2, Float2, f32) {
    let beta = 0.15;

    let (sin_a, cos_a) = rb_a.rotation.sin_cos();
    let r_a = Float2::new(
        anchor_a_local.x * cos_a - anchor_a_local.y * sin_a,
        anchor_a_local.x * sin_a + anchor_a_local.y * cos_a,
    );

    let (sin_b, cos_b) = rb_b.rotation.sin_cos();
    let r_b = Float2::new(
        anchor_b_local.x * cos_b - anchor_b_local.y * sin_b,
        anchor_b_local.x * sin_b + anchor_b_local.y * cos_b,
    );

    let world_a = rb_a.position + r_a;
    let world_b = rb_b.position + r_b;

    let positional_error = world_b - world_a;

    let v_a = rb_a.velocity_at(r_a);
    let v_b = rb_b.velocity_at(r_b);
    let relative_v = v_b - v_a;

    let linear_bias = -positional_error * (beta / FIXED_DT);

    let inv_mass_sum = rb_a.inv_mass + rb_b.inv_mass;

    let k_x =
        inv_mass_sum + (r_a.y * r_a.y * rb_a.inv_inertia) + (r_b.y * r_b.y * rb_b.inv_inertia);

    let k_y =
        inv_mass_sum + (r_a.x * r_a.x * rb_a.inv_inertia) + (r_b.x * r_b.x * rb_b.inv_inertia);

    let j_x = (-relative_v.x + linear_bias.x) / k_x.max(1e-6);
    let j_y = (-relative_v.y + linear_bias.y) / k_y.max(1e-6);

    let impulse = Float2::new(j_x, j_y);

    // angular correction
    let angular_error = (rb_b.rotation - rb_a.rotation + std::f32::consts::PI)
        .rem_euclid(2.0 * std::f32::consts::PI)
        - std::f32::consts::PI;

    let relative_w = rb_b.angular_velocity - rb_a.angular_velocity;

    let angular_bias = (-angular_error * beta / FIXED_DT).clamp(-1.0, 1.0);

    let k_ang = rb_a.inv_inertia + rb_b.inv_inertia;

    let j_ang = if k_ang > 0.0 {
        (-relative_w + angular_bias) / k_ang
    } else {
        0.0
    };

    (impulse, r_a, j_ang)
}

impl SystemBase for ConnectorSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        if FIXED_DT <= 0.0 {
            return;
        }

        // -----------------------------
        // 1. Break detection
        // -----------------------------
        world.for_each2_mut_both::<PhysicsConnector, RigidBody>(|_, connector, rb| {
            check_if_broken(connector, rb);
        });

        let mut broken = HashSet::new();

        world.for_each::<PhysicsConnector>(|e, c| {
            if !c.is_intact {
                broken.insert(e);
            }
        });

        world.for_each_mut::<PhysicsConnector>(|_, connector| {
            for (i, c) in connector.connections.iter_mut().enumerate() {
                if let Some(other) = c {
                    if broken.contains(other) {
                        *c = None;
                        connector.anchors[i] = None;
                    }
                }
            }
        });

        // -----------------------------
        // 2. Build working body cache
        // -----------------------------
        let mut bodies: HashMap<_, RigidBody> = HashMap::new();
        world.for_each::<RigidBody>(|e, rb| {
            bodies.insert(e, rb.clone());
        });

        // -----------------------------
        // 3. Iterative solver (Gauss-Seidel)
        // -----------------------------
        for _ in 0..SOLVER_ITERATIONS {
            let mut updates_linear: HashMap<_, Float2> = HashMap::new();
            let mut updates_angular: HashMap<_, f32> = HashMap::new();

            world.for_each::<PhysicsConnector>(|entity, connector| {
                if !connector.is_intact {
                    return;
                }

                let Some(rb_a) = bodies.get(&entity).cloned() else {
                    return;
                };

                for (i, &maybe_b) in connector.connections.iter().enumerate() {
                    let Some(b) = maybe_b else {
                        continue;
                    };
                    let Some(rb_b) = bodies.get(&b).cloned() else {
                        continue;
                    };
                    let Some(anchor_a) = connector.anchors[i] else {
                        continue;
                    };

                    let anchor_b = Float2::ZERO;

                    let (impulse, r_a, ang) = solve_constraint(&rb_a, &rb_b, anchor_a, anchor_b);

                    updates_linear
                        .entry(entity)
                        .and_modify(|v| *v -= impulse)
                        .or_insert(-impulse);

                    updates_linear
                        .entry(b)
                        .and_modify(|v| *v += impulse)
                        .or_insert(impulse);

                    if ang != 0.0 {
                        *updates_angular.entry(entity).or_insert(0.0) -= ang;
                        *updates_angular.entry(b).or_insert(0.0) += ang;
                    }
                }
            });

            // apply immediately into working bodies (THIS is the key fix)
            for (e, imp) in updates_linear.drain() {
                if let Some(rb) = bodies.get_mut(&e) {
                    rb.apply_impulse(imp, Float2::ZERO);
                }
            }

            for (e, j) in updates_angular.drain() {
                if let Some(rb) = bodies.get_mut(&e) {
                    rb.angular_velocity += j * rb.inv_inertia;
                }
            }
        }

        // -----------------------------
        // 4. Write back to ECS
        // -----------------------------
        for (e, rb) in bodies {
            world.get_component_mut::<RigidBody, _>(e, |dst| {
                *dst = rb;
            });
        }
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
