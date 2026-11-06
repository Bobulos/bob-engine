use crate::{Component, runtime::rendering::camera::Camera};

#[derive(Component!)]
pub struct CameraProxy {   
    pub camera_data: Camera,
}