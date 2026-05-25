use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::math::Float2;
use crate::runtime::phys::ContactManifold;
use crate::runtime::phys::collisions::{circle_circle, circle_rect, rect_rect};
use crate::runtime::phys::physics_config::PhysicsConfig;
pub use crate::runtime::phys::{Aabb, Manifold, RigidBody, Shape};
use std::sync::Arc;

pub struct PhysicsSystem {
    pub config: PhysicsConfig,
    manifolds: Vec<Manifold>,
    snapshots: Vec<(Entity, Aabb, Float2, f32, Shape)>,
}

const FIXED_DT: f32 = 1.0 / 60.0;

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            config: PhysicsConfig::default(),
            manifolds: Vec::new(),
            snapshots: Vec::new(),
        }
    }

    /// Spawn a physics-backed entity; Transform is synced from RigidBody.position.
    pub fn spawn_body(&self, world: &Arc<DynamicWorld>, body: RigidBody) -> Entity {
        let pos = body.position;
        let angle = body.angle;
        let entity = world.create_entity();
        world.add_component(
            entity,
            Transform {
                position: pos,
                angle,
            },
        );
        world.add_component(entity, body);
        entity
    }

    // ── Fixed-step sub-systems ────────────────────────────────────────────────

    fn integrate(&self, world: &Arc<DynamicWorld>, dt: f32) {
        let g = self.config.gravity;
        world.for_each_mut::<RigidBody>(|_e, body| {
            body.integrate(dt, g);
        });
    }

    /// Collect all (Entity, snapshot) pairs for broad + narrow phase.
    fn collect_snapshots(&mut self, world: &Arc<DynamicWorld>) {
        self.snapshots.clear();
        world.for_each::<RigidBody>(|entity, body| {
            self.snapshots.push((
                entity,
                body.aabb(),
                body.position,
                body.angle,
                body.shape.clone(),
            ));
        });
    }

    fn build_manifolds(&mut self, _world: &Arc<DynamicWorld>) {
        self.manifolds.clear();
        let n = self.snapshots.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (ea, aabb_a, pos_a, angle_a, ref shape_a) = self.snapshots[i];
                let (eb, aabb_b, pos_b, angle_b, ref shape_b) = self.snapshots[j];

                // Broad phase
                if !aabb_a.overlaps(aabb_b) {
                    continue;
                }

                // Narrow phase
                let result: Option<(Float2, f32, ContactManifold)> = match (shape_a, shape_b) {
                    (Shape::Circle { radius: ra }, Shape::Circle { radius: rb }) => {
                        circle_circle(pos_a, *ra, pos_b, *rb)
                    }
                    (Shape::Rect { .. }, Shape::Rect { .. }) => {
                        let va = shape_a.rect_vertices(pos_a, angle_a);
                        let vb = shape_b.rect_vertices(pos_b, angle_b);
                        rect_rect(&va, &vb, pos_a, pos_b)
                    }
                    (Shape::Circle { radius }, Shape::Rect { .. }) => {
                        let verts = shape_b.rect_vertices(pos_b, angle_b);
                        circle_rect(pos_a, *radius, &verts, pos_b).map(|(n, d, c)| (-n, d, c))
                    }
                    (Shape::Rect { .. }, Shape::Circle { radius }) => {
                        let verts = shape_a.rect_vertices(pos_a, angle_a);
                        circle_rect(pos_b, *radius, &verts, pos_a)
                    }
                };

                if let Some((normal, depth, contact_manifold)) = result {
                    self.manifolds.push(Manifold {
                        body_a: ea,
                        body_b: eb,
                        normal,
                        depth,
                        contacts: contact_manifold.points,
                        contact_count: contact_manifold.count,
                    });
                }
            }
        }
    }

    fn resolve_impulses(&self, world: &Arc<DynamicWorld>) {
        for _ in 0..self.config.iterations {
            for m in &self.manifolds {
                if m.body_a == m.body_b {
                    continue;
                }
                for &contact in &m.contacts[..m.contact_count] {
                    // Read body_a first, releasing the lock before touching body_b
                    let body_a_snapshot = world.get_component::<RigidBody, _>(m.body_a, |a| {
                        (
                            a.position,
                            a.inv_mass,
                            a.inv_inertia,
                            a.restitution,
                            a.friction,
                            a.velocity,
                            a.angular_velocity,
                        )
                    });

                    let calculation_data = match body_a_snapshot {
                        None => continue,
                        Some((
                            pos_a,
                            inv_mass_a,
                            inv_inertia_a,
                            restitution_a,
                            friction_a,
                            vel_a,
                            ang_vel_a,
                        )) => {
                            let ra = contact - pos_a;
                            let vel_at_ra = vel_a + Float2::cross_scalar_vec(ang_vel_a, ra);

                            world
                                .get_component::<RigidBody, _>(m.body_b, |b| {
                                    let rb = contact - b.position;
                                    let rel_vel = b.velocity_at(rb) - vel_at_ra;
                                    let vel_along_normal = rel_vel.dot(m.normal);
                                    if vel_along_normal > 0.0 {
                                        return None;
                                    }
                                    let e = restitution_a.min(b.restitution);
                                    let ra_cross_n = ra.cross(m.normal);
                                    let rb_cross_n = rb.cross(m.normal);
                                    let inv_mass_sum = inv_mass_a
                                        + b.inv_mass
                                        + ra_cross_n * ra_cross_n * inv_inertia_a
                                        + rb_cross_n * rb_cross_n * b.inv_inertia;
                                    if inv_mass_sum < 1e-6 {
                                        return None;
                                    }
                                    let j = -(1.0 + e) * vel_along_normal
                                        / (inv_mass_sum * m.contacts.len() as f32);
                                    let tangent = {
                                        let t = rel_vel - m.normal * rel_vel.dot(m.normal);
                                        if t.length_sq() < 1e-10 {
                                            Float2::ZERO
                                        } else {
                                            t.normalize()
                                        }
                                    };
                                    let jt = -rel_vel.dot(tangent)
                                        / (inv_mass_sum * m.contacts.len() as f32);
                                    let mu = (friction_a + b.friction) * 0.5;
                                    let friction_impulse = if jt.abs() < j.abs() * mu {
                                        tangent * jt
                                    } else {
                                        tangent * (-j * mu)
                                    };
                                    Some((j, friction_impulse, ra, rb))
                                })
                                .flatten()
                        }
                    };

                    if let Some((j, friction_impulse, ra, rb)) = calculation_data {
                        let normal_impulse = m.normal * j;
                        world.get_component_mut::<RigidBody, _>(m.body_a, |a| {
                            a.apply_impulse(-normal_impulse, ra);
                            a.apply_impulse(-friction_impulse, ra);
                        });
                        world.get_component_mut::<RigidBody, _>(m.body_b, |b| {
                            b.apply_impulse(normal_impulse, rb);
                            b.apply_impulse(friction_impulse, rb);
                        });
                    }
                }
            }
        }
    }

    fn positional_correction(&self, world: &Arc<DynamicWorld>) {
        for m in &self.manifolds {
            // Read pass: extract the inverse masses safely
            let mass_data = world
                .get_component::<RigidBody, _>(m.body_a, |a| {
                    world.get_component::<RigidBody, _>(m.body_b, |b| (a.inv_mass, b.inv_mass))
                })
                .flatten();

            if let Some((inv_mass_a, inv_mass_b)) = mass_data {
                let inv_mass_sum = inv_mass_a + inv_mass_b;
                if inv_mass_sum < 1e-10 {
                    continue;
                }

                let magnitude = (m.depth - self.config.slop).max(0.0) / inv_mass_sum
                    * self.config.correction_percent;
                let correction = m.normal * magnitude;

                // Mutate pass
                world.get_component_mut::<RigidBody, _>(m.body_a, |a| {
                    a.position -= correction * inv_mass_a;
                });
                world.get_component_mut::<RigidBody, _>(m.body_b, |b| {
                    b.position += correction * inv_mass_b;
                });
            }
        }
    }

    /// Copy RigidBody positions back to Transform components.
    fn sync_transforms(&self, world: &Arc<DynamicWorld>) {
        world.for_each2_mut_both::<RigidBody, Transform>(|_e, body, transform| {
            transform.position = body.position;
            transform.angle = body.angle;
        });
    }

    fn step(&mut self, world: &Arc<DynamicWorld>) {
        self.integrate(world, FIXED_DT);

        self.collect_snapshots(world);

        self.build_manifolds(world);

        self.resolve_impulses(world);

        self.positional_correction(world);

        self.sync_transforms(world);
    }
}

impl SystemBase for PhysicsSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        // Ground plane (static)
        let mut col = RigidBody::new_static(
            Shape::Rect {
                half_w: 3000.0,
                half_h: 0.5,
            },
            Float2::new(0.0, -5.0),
        );
        col.angle = 0.0;
        self.spawn_body(world, col);

        let mut col = RigidBody::new_static(
            Shape::Rect {
                half_w: 20.0,
                half_h: 0.5,
            },
            Float2::new(0.0, -5.0),
        );
        col.angle = 1.57079632679;
        self.spawn_body(world, col);
    }

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        self.step(world);
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
