use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::instance::Instance;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use std::sync::{Arc, RwLock};

// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct RenderSystem {
    renderer: Arc<RwLock<Renderer>>,
}
impl RenderSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self { renderer: renderer }
    }
}
impl SystemBase for RenderSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut renderer_lock = self.renderer.write().unwrap();
        world.for_each2_mut::<Transform, Sprite>(
            |_entity: Entity, transform: &mut Transform, sprite: &Sprite| {
                // Don't render unitialized sprites
                if sprite.index != usize::MAX && sprite.visible {
                    renderer_lock.batches[sprite.batch_index].instances[sprite.index] = Instance {
                        position: transform.position.into(),
                        size: [1.0, 1.0],
                        uv_offset: [0.0, 0.0],
                        uv_scale: [1.0, 1.0],
                        rotation: transform.rotation,
                    };
                }
            },
        );
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
