use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::instance::SpriteInstance;
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
                if sprite.index != usize::MAX && sprite.visible {
                    let batch = &mut renderer_lock.batches[sprite.batch_index];
                    let instances: &mut [SpriteInstance] = bytemuck::cast_slice_mut(&mut batch.instances);
                    instances[sprite.index] = SpriteInstance {
                        position: transform.position.into(),
                        size: [1.0, 1.0],
                        uv_offset: sprite.uv_offset,
                        uv_scale: sprite.uv_scale,
                        rotation: transform.rotation,
                    };
                }
            },
        );
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
