use crate::runtime::ecs::Entity;
use crate::runtime::math::Float2;

#[derive(Debug, Clone)]
pub struct Manifold {
    /// Points to body in the snapshot stack
    pub body_a: usize,
    /// Points to body in the snapshot stack
    pub body_b: usize,
    pub normal: Float2,
    pub depth: f32,
    pub contacts: [Float2; 8],
    pub contact_count: usize,
}
