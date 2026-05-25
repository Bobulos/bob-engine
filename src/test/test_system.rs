use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::Float2;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
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
        for y in 10..100 {
            for x in -20..20 {
                let x = (x * 4) as f32;
                let y = (y * 4) as f32;
                let e = world.create_entity();
                world.add_component(
                    e,
                    Transform {
                        position: Float2::new(x, y),
                        angle: 0.0,
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
                    Float2::new(x, y),
                );
                rb.inv_mass = 1.0;
                rb.velocity = Float2::new(1.0, 1.0);
                world.add_component(e, rb);
            }
        }
    }
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        // self.accumulator += 1;
        // if self.accumulator % 4 == 0 {
        //     for x in -2..2 {
        //         let x = (x * 4) as f32;
        //         let e = world.create_entity();
        //         world.add_component(
        //             e,
        //             Transform {
        //                 position: Float2::new(x as f32, 30.0),
        //                 angle: 0.0,
        //             },
        //         );
        //         world.add_component(
        //             e,
        //             Sprite {
        //                 visible: true,
        //                 batch_index: 0,
        //                 index: usize::MAX,
        //                 atlas_id: 0,
        //                 width: 1,
        //                 height: 1,
        //             },
        //         );
        //         let mut rb = crate::b_engine::physics_systems::RigidBody::new(
        //             crate::b_engine::physics_systems::Shape::Circle { radius: 0.5 },
        //             1.0,
        //             Float2::new(x as f32, 30.0),
        //         );
        //         rb.inv_mass = 1.0;
        //         rb.velocity = Float2::new(1.0, 1.0);
        //         world.add_component(e, rb);
        //     }
        // }

        // for _ in 0..20 {
        //     self.spawned += 1;
        //     let spawned = self.spawned as f32 * 0.01;
        //     let position = Float2 {
        //         x: spawned * f32::sin(spawned),
        //         y: spawned * f32::cos(spawned),
        //     };

        //     let e = world.create_entity();
        //     world.add_component(
        //         e,
        //         core_components::Sprite {
        //             visible: true,
        //             batch_index: 0,
        //             index: usize::MAX,
        //             atlas_id: 0,
        //             width: 1,
        //             height: 1,
        //         },
        //     );

        //     world.add_component(
        //         e,
        //         core_components::Transform {
        //             position,
        //             angle: 0.0,
        //         },
        //     );
        // }

        // const SPEED: f32 = 0.01;
        // world.for_each_mut::<Transform>(|_entity: Entity, transform: &mut Transform| {
        //     let dir = Float2::ZERO - transform.position;
        //     dir.normalize();
        //     transform.position += dir * SPEED;
        // });
    }

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
