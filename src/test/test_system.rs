use crate::app::WINDOW_SIZE;
use crate::runtime::assets::AssetHandle;
use crate::runtime::assets::AssetStore;
use crate::runtime::ecs::Entity;
use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::math::{self, Float2};
use crate::runtime::phys::connector::PhysCxn;
use crate::runtime::rendering::BatchHandle;
use crate::runtime::rendering::Color;
use crate::runtime::rendering::gui_rendering::GuiShape;
use crate::runtime::rendering::gui_rendering::components::gui_shape::Border;
use crate::runtime::rendering::gui_rendering::components::gui_transform::GuiTransform;
use crate::runtime::rendering::renderer::PipelineKey;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use std::sync::{Arc, OnceLock};
// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;

pub struct TestSystem {
    asset_store: Arc<OnceLock<AssetStore>>,
    ship_handle: Option<AssetHandle>,
    proj_handle: Option<AssetHandle>,
    ui: Option<AssetHandle>,
}
impl TestSystem {
    pub fn new(asset_store: Arc<OnceLock<AssetStore>>) -> Self {
        Self {
            asset_store,
            ui: None,
            ship_handle: None,
            proj_handle: None,
        }
    }
    pub fn test_physics(&mut self, world: &Arc<DynamicWorld>) {
        if let Some(asset_store) = self.asset_store.get() {
            self.ship_handle = asset_store.get_asset_handle_by_path("exp/ship_parts_s.png");
            self.proj_handle = asset_store.get_asset_handle_by_path("exp/projectiles_m.png");
            self.ui = asset_store.get_asset_handle_by_path("default_ui/default_bck.png")
        }

        let targ = Float2::new(5.0, 0.0);

        if let Some(_sprite_handle) = self.ship_handle {
            let ship_sprite =
                Sprite::new(true, [0.0, 0.0], [1.0 / 6.0, 1.0]);
            let proj_sprite = Sprite::new(
                true,
                [0.5, 0.0],
                [0.5, 1.0],
            );
            let batch_ship_handle = BatchHandle::new(self.ship_handle.unwrap(), PipelineKey::Sprite);
            let batch_proj_handle = BatchHandle::new(self.proj_handle.unwrap(), PipelineKey::Sprite);

            const TEST_MASS: f32 = 0.01;
            const TEST_VEL: f32 = 5.0;
            for _ in 0..1000 {
                let e = world.create_entity();
                let pos = Float2::new(
                    rand::random::<f32>() * 2000.0 - 1000.0,
                    rand::random::<f32>() * 2000.0 - 1000.0,
                );

                //let pos = Float2::new(-1000.0, 0.0);
                //let pos = Float2::new(5.0, -100.0);
                let rot = math::angle_to_point(pos, targ) + std::f32::consts::PI / 2.0;
                world.add_component_safe(
                    e,
                    Transform {
                        position: pos,
                        rotation: rot,
                    },
                );
                world.add_component_safe(e, proj_sprite);
                world.add_component_safe(e, batch_proj_handle);
                let mut rb = crate::runtime::phys::RigidBody::new(
                    crate::runtime::phys::Shape::Circle { radius: 0.5 },
                    TEST_MASS,
                    pos,
                    rot,
                );

                rb.velocity = (targ - pos).normalize() * TEST_VEL;
                world.add_component_safe(e, rb);
            }

            const LENGTH: usize = 10;

            for y in 0..1 {
                let mut bodies: Vec<Entity> = Vec::new();
                for _ in 0..LENGTH {
                    bodies.push(world.create_entity());
                }
                for x in 0..LENGTH {
                    let entity = bodies[x];

                    let mut cxn_a: Option<PhysCxn> = None;
                    let mut cxn_b: Option<PhysCxn> = None;

                    // Link to the PREVIOUS entity: The anchor should be on our LEFT side (-0.5)
                    if x > 0 {
                        cxn_a = Some(PhysCxn::new(bodies[x - 1], Float2::new(-1.0, 0.0)));
                    }

                    // Link to the NEXT entity: The anchor should be on our RIGHT side (0.5)
                    if x < LENGTH - 1 {
                        cxn_b = Some(PhysCxn::new(bodies[x + 1], Float2::new(1.0, 0.0)));
                    }

                    let pos = Float2::new(x as f32, 5.0 * y as f32);

                    world.add_component_safe(
                        entity,
                        Transform {
                            position: pos,
                            rotation: 0.0,
                        },
                    );
                    world.add_component_safe(entity, batch_ship_handle);
                    world.add_component_safe(entity, ship_sprite.clone());

                    world.add_component_safe(
                        entity,
                        crate::runtime::phys::RigidBody::new(
                            crate::runtime::phys::Shape::Rect {
                                half_w: 0.5,
                                half_h: 0.5,
                            },
                            1.0,
                            pos,
                            0.0,
                        ),
                    );

                    world.add_component_safe(
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
    }
    pub fn test_gui(&mut self, world: &Arc<DynamicWorld>) {
        let size = crate::app::WINDOW_SIZE;
        //1080 / 720
        for x in 0..10 {
            let entity = world.create_entity();
            world.add_component_safe(entity, BatchHandle::new(self.ui.unwrap(), PipelineKey::Gui));
            world.add_component_safe(entity, GuiTransform::new(Float2::new(x as f32*200.0, 1080.0-200.0)));
            world.add_component_safe(entity, GuiShape::new(
                true, 
                [200.0, 200.0], 
                Color::from_hex("#00909E").unwrap(),
                Border::Bordered(Color::from_hex("#006884").unwrap(), 20.0, 20.0),
                //Border::Borderless,
                [0.0, 0.0], [1.0, 1.0]));
            }

        // world.add_component_safe(entity, BatchHandle::new(self.ui.unwrap(), PipelineKey::Gui));
        // world.add_component_safe(entity, GuiTransform::new(Float2::new(0.0, 720.0-200.0)));
        // world.add_component_safe(entity, GuiShape::new(
        //     true, 
        //     [200.0, 200.0], 
        //     Color::from_hex("#00909E").unwrap(), 
        //     Color::from_hex("#006884").unwrap(), 
        //     20.0, 20.0, [0.0, 0.0], [1.0, 1.0]));
    }
}
impl SystemBase for TestSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        self.test_physics(world);
        self.test_gui(world);
    }
    fn on_update(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
