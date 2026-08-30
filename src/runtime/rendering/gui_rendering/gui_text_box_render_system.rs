use crate::runtime::math::Float2;
use crate::runtime::rendering::Color;
use crate::runtime::ecs::{DynamicWorld, Entity, SystemBase};
use crate::runtime::gui::components::{gui_shape::GuiShape, gui_transform::GuiTransform};
use crate::runtime::rendering::gui_rendering::GuiCharInstance;
use crate::runtime::rendering::gui_rendering::gui_shape_instance::GuiShapeInstance;
use crate::runtime::gui::components::GuiBorder;
use crate::runtime::rendering::Renderer;
use crate::runtime::rendering::BatchHandle;
use crate::runtime::gui::components::GuiTextBox;
use std::sync::{Arc, RwLock};

// #[path = "../engine//ecs/component_store.rs"]
// mod component_store;
pub struct GuiTextRenderSystem {
    renderer: Arc<RwLock<Renderer>>,
}
impl GuiTextRenderSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self { renderer: renderer }
    }
}

impl SystemBase for GuiTextRenderSystem {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut renderer_lock = self.renderer.write().unwrap();


        // this doesn't need to use the batch handle instance
        // track char instance seperately
        let mut i: usize = 0;
        world.for_each3_mut::<GuiTextBox, GuiTransform, BatchHandle>(
            |_entity, txt, transform, batch_handle| {
                if batch_handle.index != usize::MAX && txt.dirty && txt.visible {

                    //println!("running text rendering");
                    txt.dirty = false;
                    let batch = &mut renderer_lock.batches[batch_handle.batch_index];
                    //println!("{} stride, {} cap, {} len", batch.instance_stride, batch.instance_capacity, batch.instances.len());
                    let chars = txt.generate_char_instances(transform.position);
                    //println!("{:?}",chars);
                    let instances: &mut [GuiCharInstance] = bytemuck::cast_slice_mut(&mut batch.instances);

                    for c in chars.iter() {
                        instances[i] = *c;
                        i += 1;
                    }
                    // 
                    // ALL THIS NEEDS TO DO IS DUMP THE 
                    // CREATED INSTANCE INTO THE BUFFER
                }
            },
        );
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}