use crate::runtime::math::Float2;
use crate::runtime::rendering::Color;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::gui::components::{gui_shape::GuiShape, gui_transform::GuiTransform};
use crate::runtime::rendering::gui_rendering::gui_shape_instance::GuiShapeInstance;
use crate::runtime::gui::components::GuiBorder;
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::BatchHandle;
use std::sync::{Arc, RwLock};

// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct GuiShapeRenderSystem {
    renderer: Arc<RwLock<Renderer>>,
}
impl GuiShapeRenderSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self { renderer: renderer }
    }
}

const MAX_CLEAN_PER_FRAME: usize = 8;
impl SystemBase for GuiShapeRenderSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut renderer_lock = self.renderer.write().unwrap();

        world.for_each3_mut::<GuiShape, GuiTransform, BatchHandle>(
            |_entity, shape, transform, batch_handle| {
                if batch_handle.index != usize::MAX && shape.visible {

                    if !shape.dirty {
                        return;
                    }
                    shape.dirty = false;
                    
                    let batch = &mut renderer_lock.batches[batch_handle.batch_index];
                    let instances: &mut [GuiShapeInstance] = bytemuck::cast_slice_mut(&mut batch.instances);

                    let mut border_color: Color = Color::transparent(); 
                    let mut border_width: f32 = 0.0;
                    let mut border_radius: f32 = 0.0;
                    match shape.border {
                        GuiBorder::Bordered(color, width, radius) => {
                            border_color = color;
                            border_radius = radius;
                            border_width = width;
                        } 
                        GuiBorder::Borderless => {}
                    }
                    let position = transform.position;                    
                    //let position = transform.position;
                    instances[batch_handle.index] = GuiShapeInstance {
                        position: position.into(),
                        size: transform.size.into(),
                        uv_offset: shape.uv_offset,
                        uv_scale: shape.uv_scale,
                        color: shape.fill_color.value,
                        corner_radius: border_radius,
                        border_color: border_color.value,
                        border_width: border_width,
                        _pad: [0; 3],
                    };
                }
            },
        );
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}