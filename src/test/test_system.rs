use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::{Float2, float2};
use crate::runtime::phys::RigidBody;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use rand::Rng;
use std::sync::Arc;
// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct TestSystem {
    spawned: u32,
    targets: Vec<Float2>,
    accumulator: i32,
}
impl TestSystem {
    pub fn new() -> Self {
        Self {
            spawned: 0,
            targets: Vec::new(),
            accumulator: 0,
        }
    }
}
const GRAVITY: f32 = 9.8;
const ENTITY_COUNT: usize = 10000;
impl SystemBase for TestSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        for _ in 0..100 {
            let e = world.create_entity();
            let pos = Float2::new(
                rand::random::<f32>() * 2000.0,
                rand::random::<f32>() * 2000.0,
            );
            world.add_component(
                e,
                Transform {
                    position: pos,
                    rotation: 0.0,
                },
            );
            world.add_component(
                e,
                Sprite {
                    visible: true,
                    batch_index: 0,
                    index: usize::MAX,
                    atlas_id: 0,
                    width: 1,
                    height: 1,
                },
            );
            let mut rb = crate::runtime::phys::RigidBody::new(
                crate::runtime::phys::Shape::Rect {
                    half_w: 3.0,
                    half_h: 1.0,
                },
                100.0,
                pos,
            );

            rb.velocity = (Float2::new(0.0, 60.0) - pos).normalize() * 60.0;
            world.add_component(e, rb);
        }

        for y in 10..100 {
            for x in -40..40 {
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
                    Sprite {
                        visible: true,
                        batch_index: 0,
                        index: usize::MAX,
                        atlas_id: 0,
                        width: 1,
                        height: 1,
                    },
                );
                let mut rb = crate::runtime::phys::RigidBody::new(
                    crate::runtime::phys::Shape::Circle { radius: 0.5 },
                    1.0,
                    pos,
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
