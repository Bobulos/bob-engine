use crate::float2::Float2;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Float2,
    pub angle: f32,
}

impl Transform {
    pub fn new(position: Float2, angle: f32) -> Self {
        Self { position, angle }
    }
}
