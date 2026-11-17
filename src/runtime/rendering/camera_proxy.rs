use crate::{Component, runtime::rendering::camera::Camera};

#[derive(Component!)]
pub struct CameraProxy {   
    pub is_main_camera: bool,
    pub camera_data: Camera,
}
// 
// impl Default for CameraProxy {
//     fn default() -> Self {
//         Self { 
//             is_main_camera: true, 
//             camera_data: Camera::default() 
//         }
//     }
// }