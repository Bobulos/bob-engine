use crate::runtime::math::Float2;
use crate::Component;

#[derive(Component!)]
pub struct GuiTransform {
    pub position: Float2,
}

impl GuiTransform {
    pub fn new(position: Float2) -> Self {
        Self { position }
    }
}