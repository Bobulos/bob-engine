use crate::runtime::ecs::Entity;
use crate::runtime::math::Float2;

#[derive(Debug, Clone)]
pub struct Manifold {
    pub body_a: Entity,
    pub body_b: Entity,
    pub normal: Float2,
    pub depth: f32,
    pub contacts: [Float2; 8],
    pub contact_count: usize,
}
