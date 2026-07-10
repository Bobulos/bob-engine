use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::rendering::gui_rendering::GuiShape;
use crate::runtime::rendering::gui_rendering::components::gui_transform::GuiTransform;
use crate::runtime::rendering::gui_rendering::gui_instance::GuiInstance;
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::BatchHandle;
use std::sync::{Arc, RwLock};

// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct GuiRenderSystem {
    renderer: Arc<RwLock<Renderer>>,
}
impl GuiRenderSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self { renderer: renderer }
    }
}
impl SystemBase for GuiRenderSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut renderer_lock = self.renderer.write().unwrap();
        world.for_each3::<GuiTransform, GuiShape, BatchHandle>(
            |_entity: Entity, transform: &GuiTransform, shape: &GuiShape, batch_handle: &BatchHandle| {
                if batch_handle.index != usize::MAX && shape.visible {
                    let batch = &mut renderer_lock.batches[batch_handle.batch_index];
                    let instances: &mut [GuiInstance] = bytemuck::cast_slice_mut(&mut batch.instances);
                    instances[batch_handle.index] = GuiInstance {
                        position: transform.position.into(),
                        size: shape.size,
                        uv_offset: shape.uv_offset,
                        uv_scale: shape.uv_scale,
                        color: shape.fill_color.value,
                        corner_radius: shape.corner_radius,
                        border_color: shape.border_color.value,
                        border_width: shape.border_width,
                        _pad: [0; 3],
                    };
                }
            },
        );
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}