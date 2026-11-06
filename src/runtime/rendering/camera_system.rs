use crate::runtime::{ecs::SystemBase, rendering::Renderer};
use std::sync::{Arc, RwLock};

pub struct CameraSystem {
    renderer: Arc<RwLock<super::Renderer>>,
}
impl CameraSystem {
    pub fn new(renderer: Arc<RwLock<super::Renderer>>) -> Self {
        Self { renderer }
    }
}
impl SystemBase for CameraSystem {
    fn on_start(&mut self, world: &Arc<crate::runtime::ecs::DynamicWorld>) {
        // Initialize camera buffer if needed
        let renderer_lock = self.renderer.read();
    }
    fn on_update(&mut self, world: &Arc<crate::runtime::ecs::DynamicWorld>) {
        // Update camera matrix and queue render pass
        let mut renderer = self.renderer.write().unwrap();
        renderer.update_camera();
    }
    fn on_destroy(&mut self, world: &Arc<crate::runtime::ecs::DynamicWorld>) {
        // Cleanup if needed
    }
}
