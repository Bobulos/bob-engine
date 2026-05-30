use crate::runtime::math::Float2;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Float2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Float2, rotation: f32) -> Self {
        Self { position, rotation }
    }
}
