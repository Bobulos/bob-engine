use crate::runtime::Input;
use crate::runtime::assets::{AssetEmbedded, AssetStore};
use crate::runtime::ecs::DynamicWorld;
use crate::runtime::ecs::SystemGroup;
use crate::runtime::ecs::entities::Entities;
use crate::runtime::ecs::system_group::SystemGroupThreading;
use crate::runtime::rendering;
use crate::runtime::rendering::Renderer;
use crate::runtime::scene::world_serializer;

use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;
use std::time::Instant;
pub struct Engine {
    pub frame_count: u64,
    pub renderer: Arc<RwLock<Renderer>>,
    pub input: Arc<RwLock<Input>>,
    pub entities: Entities,
    // Assets
    pub asset_store: Arc<OnceLock<AssetStore>>,
}

pub const MAIN_WORLD: &str = "main";
pub const RENDER_GROUP: &str = "render_group";
pub const PHYSICS_GROUP: &str = "physics_group";
pub const PHYSICS_CONNECTION_GROUP: &str = "physics_connection_group";
pub const SPRITE_BATCH_SIZE: usize = 1024 * 4; // 2^10
pub const FIXED_DT: f32 = 1.0 / 60.0; // 2^14
// pub const INCLUDE_ATLAS: &[&str] = &[
//     "tree.png",
//     "Tux.png",
//     "exp/ship_parts_s.png",
//     "exp/projectiles_m.png",
// ];
impl Engine {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            frame_count: 0,
            renderer: Arc::new(RwLock::new(renderer)),
            input: Arc::new(RwLock::new(Input::new())),
            entities: Entities::new(),
            asset_store: Arc::new(OnceLock::new()),
        }
    }

    pub fn init(&mut self) {
        let mut asset_store = AssetStore::new();
        asset_store.init();
        self.asset_store.set(asset_store).expect("One lock sucks");
        self.renderer
            .write()
            .unwrap()
            .init(self.asset_store.clone());
        self.setup_world();
        self.setup_renderer();
        self.setup_systems();

        self.entities
            .worlds
            .get(MAIN_WORLD)
            .unwrap()
            .get_included_components();
        println!("Engine initialized");
    }

    fn setup_world(&mut self) {
        self.entities
            .add_world(MAIN_WORLD, Arc::new(DynamicWorld::new()));
        crate::runtime::ecs::system_bootstrap::bootstrap(&self);
    }

    fn setup_renderer(&mut self) {
        self.setup_sprites();
        self.setup_tilemap();
    }

    fn setup_sprites(&mut self) {
        let _world = self.entities.get_world(MAIN_WORLD).unwrap();
    }

    fn setup_tilemap(&mut self) {
        let tilemap = [0u8; 64 * 64];
        let file = AssetEmbedded::get("grass.png").unwrap();
        let bytes: &[u8] = &file.data;
        let test = AssetEmbedded::get("test.png").unwrap();
        let test_bytes: &[u8] = &test.data;
        // Acquire the lock once to do all tilemap work — avoids the deadlock
        // that occurs when holding a write guard and calling .queue() via a
        // second write() on the same RwLock in the same expression.
        let mut renderer = self.renderer.write().unwrap();
        let trees = renderer.create_tilemap(test_bytes, &tilemap, 64, 64, 32);
        renderer.tilemaps[trees].move_by(0.0, -0.5);
        let queue = renderer.queue();
        renderer.tilemaps[trees].flush_position(queue);
        let trees = renderer.create_tilemap(bytes, &tilemap, 64, 64, 100);
        renderer.tilemaps[trees].move_by(0.0, 0.0);
        let queue = renderer.queue();
        renderer.tilemaps[trees].flush_position(queue);

        let trees = renderer.create_tilemap(bytes, &tilemap, 64, 64, 100);
        renderer.tilemaps[trees].move_by(65.0, 0.0);
        let queue = renderer.queue();
        renderer.tilemaps[trees].flush_position(queue);

        let trees = renderer.create_tilemap(bytes, &tilemap, 64, 64, 100);
        renderer.tilemaps[trees].move_by(65.0, 65.0);
        let queue = renderer.queue();
        renderer.tilemaps[trees].flush_position(queue);

        let trees = renderer.create_tilemap(bytes, &tilemap, 64, 64, 100);
        renderer.tilemaps[trees].move_by(0.0, 65.0);
        let queue = renderer.queue();
        renderer.tilemaps[trees].flush_position(queue);
    }

    fn setup_systems(&mut self) {
        println!("Initializing system groups");

        self.setup_rendering();
        self.setup_test();
        self.setup_physics();
        // initialize them jhons
        self.entities.start_system_groups();
    }

    pub fn run(&mut self) {
        self.frame_count += 1;

        let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);
        let frame_start = Instant::now();
        self.update();
        self.render().expect("Renderer fatal error");

        let elapsed = Instant::now() - frame_start;
        //print!("\rFrame time: {:.2} ms", elapsed.as_secs_f64() * 1000.0);
        if elapsed > target_frame_time {
            println!("Engine running at reduced clock");
        }
        if self.frame_count % 60 == 0 {
            println!(
                "Entity count: {}",
                self.entities.get_world(MAIN_WORLD).unwrap().entity_count()
            );
        }

        let elapsed = frame_start.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }
    const CAMERA_SPEED: f32 = 0.1;
    pub fn player_loop(&mut self) {
        let input = self.input.read().unwrap();

        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::ArrowLeft,
        )) {
            self.renderer
                .write()
                .unwrap()
                .camera
                .move_by(-Self::CAMERA_SPEED, 0.0);
        }
        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::ArrowRight,
        )) {
            self.renderer
                .write()
                .unwrap()
                .camera
                .move_by(Self::CAMERA_SPEED, 0.0);
        }
        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::ArrowUp,
        )) {
            self.renderer
                .write()
                .unwrap()
                .camera
                .move_by(0.0, Self::CAMERA_SPEED);
        }
        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::ArrowDown,
        )) {
            self.renderer
                .write()
                .unwrap()
                .camera
                .move_by(0.0, -Self::CAMERA_SPEED);
        }
        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::Digit1,
        )) {
            self.renderer.write().unwrap().camera.zoom_by(1.01);
            self.renderer.write().unwrap().update_camera();
        }
        if input.get_key_down(winit::keyboard::PhysicalKey::Code(
            winit::keyboard::KeyCode::Digit2,
        )) {
            self.renderer.write().unwrap().camera.zoom_by(0.99);
            self.renderer.write().unwrap().update_camera();
        }
    }
    pub fn update(&mut self) {
        self.update_entities();

        self.player_loop();
        // FLUSH AT END
        self.input.write().unwrap().flush(); // Clear per-frame input state at the start of the frame
    }
    pub fn render(&mut self) -> Result<(), String> {
        self.renderer
            .write()
            .unwrap()
            .render()
            .expect("Fatal error from renderer");
        Ok(())
    }
    fn update_entities(&mut self) {
        self.entities.update_system_groups();
        world_serializer::dump_entitys(&self.entities.get_world(MAIN_WORLD).unwrap());
    }

    // Setup bs
    fn setup_physics(&mut self) {
        let fetched_world = self.entities.get_world(MAIN_WORLD).unwrap();
        self.entities.add_system_group(
            PHYSICS_CONNECTION_GROUP,
            SystemGroup::new(fetched_world, SystemGroupThreading::Parallel),
        );
        let group = self
            .entities
            .get_system_group_mut(PHYSICS_CONNECTION_GROUP)
            .unwrap();
        group.register_system(
            Box::new(crate::runtime::phys::connector::connector_system::ConnectorSystem::new()),
            0,
        );

        let fetched_world = self.entities.get_world(MAIN_WORLD).unwrap();
        self.entities.add_system_group(
            PHYSICS_GROUP,
            SystemGroup::new(fetched_world, SystemGroupThreading::Parallel),
        );
        let group = self
            .entities
            .get_system_group_mut(PHYSICS_CONNECTION_GROUP)
            .unwrap();
        group.register_system(
            Box::new(crate::runtime::phys::physics_system::PhysicsSystem::new()),
            0,
        );
    }
    fn setup_test(&mut self) {
        let fetched_world = self.entities.get_world(MAIN_WORLD).unwrap();
        self.entities.add_system_group(
            "test_group",
            SystemGroup::new(fetched_world, SystemGroupThreading::Parallel),
        );
        let group = self.entities.get_system_group_mut("test_group").unwrap();
        group.register_system(
            Box::new(crate::test::test_system::TestSystem::new(Arc::clone(
                &self.asset_store,
            ))),
            0,
        );
    }
    fn setup_rendering(&mut self) {
        let fetched_world = self.entities.get_world(MAIN_WORLD).unwrap();
        self.entities.add_system_group(
            RENDER_GROUP,
            SystemGroup::new(fetched_world, SystemGroupThreading::Parallel),
        );
        // Render system
        let group = self.entities.get_system_group_mut(RENDER_GROUP).unwrap();
        let _rendering_system = group.register_system(
            Box::new(
                rendering::sprite_rendering::render_system::RenderSystem::new(Arc::clone(
                    &self.renderer,
                )),
            ),
            i32::MIN + 1,
        );
        let _rendering_system = group.register_system(
            Box::new(
                rendering::sprite_rendering::sprite_batch_allocator_system::SpriteBatchAllocatorSystem::new(
                    Arc::clone(&self.renderer)
                ),
            ),
            i32::MIN,
        );
    }
}
