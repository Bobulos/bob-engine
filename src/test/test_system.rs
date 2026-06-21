use crate::runtime::assets::AssetHandle;
use crate::runtime::assets::AssetStore;
use crate::runtime::ecs::Entity;
use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::{self, Float2};
use crate::runtime::phys::connector::PhysCxn;
use crate::runtime::phys::connector::PhysJoint;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use std::sync::{Arc, OnceLock};
use std::vec;
// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;

pub struct TestSystem {
    asset_store: Arc<OnceLock<AssetStore>>,
    sprite_handle: Option<AssetHandle>,
    other_handle: Option<AssetHandle>,
}
impl TestSystem {
    pub fn new(asset_store: Arc<OnceLock<AssetStore>>) -> Self {
        Self {
            asset_store,
            sprite_handle: None,
            other_handle: None,
        }
    }
}
impl SystemBase for TestSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        if let Some(asset_store) = self.asset_store.get() {
            self.sprite_handle = asset_store.get_asset_idx_by_path("exp/ship_parts_s.png");
            self.other_handle = asset_store.get_asset_idx_by_path("exp/projectiles_m.png");
        }

        let targ = Float2::new(0.0, 0.0);

        if let Some(sprite_handle) = self.sprite_handle {
            let sprite_cmpt =
                Sprite::new(sprite_handle, 32, 32, true, [0.0, 0.0], [1.0 / 6.0, 1.0]);
            let other_cmpt = Sprite::new(
                self.other_handle.unwrap(),
                32,
                32,
                true,
                [0.5, 0.0],
                [0.5, 1.0],
            );
            const TEST_VEL: f32 = 50.0;
            for _ in 0..1000 {
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
                world.add_component(e, other_cmpt);
                let mut rb = crate::runtime::phys::RigidBody::new(
                    crate::runtime::phys::Shape::Circle { radius: 0.5 },
                    100.0,
                    pos,
                    rot,
                );

                rb.velocity = (targ - pos).normalize() * TEST_VEL;
                world.add_component(e, rb);
            }

            const LENGTH: usize = 20;
            let mut bodies: Vec<Entity> = Vec::new();
            for i in 0..LENGTH {
                bodies.push(world.create_entity());
            }
            for x in 0..LENGTH {
                let entity = bodies[x];
                bodies.push(entity);

                let mut cxn_a: Option<PhysCxn> = None;
                let mut cxn_b: Option<PhysCxn> = None;
                if x != 0 {
                    cxn_a = Some(PhysCxn::new(bodies[x - 1], Float2::new(-1.0, 0.0)));
                }
                if x != LENGTH {
                    cxn_b = Some(PhysCxn::new(bodies[x + 1], Float2::new(1.0, 0.0)));
                }

                let pos = Float2::new(x as f32, 0.0);
                world.add_component(
                    entity,
                    Transform {
                        position: pos,
                        rotation: 0.0,
                    },
                );

                world.add_component(entity, sprite_cmpt.clone());

                world.add_component(
                    entity,
                    crate::runtime::phys::RigidBody::new(
                        crate::runtime::phys::Shape::Rect {
                            half_w: 0.5,
                            half_h: 0.5,
                        },
                        100.0,
                        pos,
                        0.0,
                    ),
                );
                world.add_component(
                    entity,
                    crate::runtime::phys::connector::PhysJoint::new(
                        10.0,
                        10.0,
                        [cxn_a, cxn_b, None, None],
                    ),
                );
            }
        }
    }
    fn on_update(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
