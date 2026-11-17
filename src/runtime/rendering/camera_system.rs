use crate::runtime::{ecs::SystemBase, rendering::{CameraProxy, Renderer, camera::Camera}};
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
        let mut cam_data: Option<Camera> = None;
        world.for_each::<CameraProxy>(|entity, camera_proxy| {
            if camera_proxy.is_main_camera {
                cam_data = Some(camera_proxy.camera_data);
            }
        });
        // Update camera matrix and queue render pass
        if let Some(d) = cam_data {
            let mut renderer = self.renderer.write().unwrap();
            renderer.write_camera_data(d);
        }
    }
    fn on_destroy(&mut self, world: &Arc<crate::runtime::ecs::DynamicWorld>) {
        // Cleanup if needed
    }
}
