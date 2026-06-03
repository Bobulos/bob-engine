use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::{self, Float2};
use crate::runtime::rendering::sprite_rendering::components::{Sprite, sprite};
use std::sync::Arc;
// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;

pub struct TestSystem {}
impl TestSystem {
    pub fn new() -> Self {
        Self {}
    }
}
const GRAVITY: f32 = 9.8;
const ENTITY_COUNT: usize = 10000;
impl SystemBase for TestSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        let targ = Float2::new(0.0, 0.0);

        for _ in 0..400 {
            let e = world.create_entity();
            let pos = Float2::new(
                rand::random::<f32>() * 2000.0 - 1000.0,
                rand::random::<f32>() * 2000.0 - 1000.0,
            );
            let rot = math::angle_to_point(pos, targ) + std::f32::consts::PI / 2.0;
            world.add_component(
                e,
                Transform {
                    position: pos,
                    rotation: rot,
                },
            );
            world.add_component(e, Sprite::new(3, 32, 32, true, [0.0, 0.0], [1.0, 1.0]));
            let mut rb = crate::runtime::phys::RigidBody::new(
                crate::runtime::phys::Shape::Circle { radius: 0.5 },
                100.0,
                pos,
                rot,
            );

            rb.velocity = (targ - pos).normalize() * 60.0;
            world.add_component(e, rb);
        }

        // Random debris
        for y in 10..20 {
            for x in 10..20 {
                let x = (x * 4) as f32;
                let y = (y * 4) as f32;
                let e = world.create_entity();
                let pos = Float2::new(x, y);
                world.add_component(
                    e,
                    Transform {
                        position: pos,
                        rotation: 0.0,
                    },
                );
                world.add_component(
                    e,
                    Sprite::new(2, 32, 32, true, [2.0 / 4.0, 0.0], [1.0 / 4.0, 1.0]),
                );

                let mut rb = crate::runtime::phys::RigidBody::new(
                    crate::runtime::phys::Shape::Rect {
                        half_w: 0.5,
                        half_h: 0.5,
                    },
                    1.0,
                    pos,
                    0.0,
                );
                rb.inv_mass = 1.0;

                rb.velocity = (Float2::new(0.0, 60.0) - pos).normalize();
                world.add_component(e, rb);
            }
        }
        for y in -10..10 {
            for x in -10..10 {
                let x = (x * 4) as f32;
                let y = ((y + 1) * 4) as f32;
                let e = world.create_entity();
                let pos = Float2::new(x, y);
                world.add_component(
                    e,
                    Transform {
                        position: pos,
                        rotation: 0.0,
                    },
                );
                world.add_component(
                    e,
                    Sprite::new(2, 32, 32, true, [1.0 / 4.0, 0.0], [1.0 / 4.0, 1.0]),
                );

                let mut rb = crate::runtime::phys::RigidBody::new(
                    crate::runtime::phys::Shape::Circle { radius: 0.5 },
                    1.0,
                    pos,
                    0.0,
                );
                rb.inv_mass = 1.0;

                rb.velocity = (Float2::new(0.0, 60.0) - pos).normalize();
                world.add_component(e, rb);
            }
        }
    }
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {}

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
