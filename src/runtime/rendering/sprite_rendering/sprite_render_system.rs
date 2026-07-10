use crate::runtime::ecs::core_components::Transform;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::sprite_rendering::SpriteInstance;
use crate::runtime::rendering::BatchHandle;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use std::sync::{Arc, RwLock};

// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct SpriteRenderSystem {
    renderer: Arc<RwLock<Renderer>>,
}
impl SpriteRenderSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self { renderer: renderer }
    }
}
impl SystemBase for SpriteRenderSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut renderer_lock = self.renderer.write().unwrap();
        world.for_each3::<Transform, Sprite, BatchHandle>(
            |_entity: Entity, transform: &Transform, sprite: &Sprite, batch_handle: &BatchHandle| {
                if batch_handle.index != usize::MAX && sprite.visible {
                    let batch = &mut renderer_lock.batches[batch_handle.batch_index];
                    let instances: &mut [SpriteInstance] = bytemuck::cast_slice_mut(&mut batch.instances);
                    instances[batch_handle.index] = SpriteInstance {
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
