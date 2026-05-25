use crate::{runtime::entities::Entity, float2::Float2};

#[derive(Debug, Clone)]
pub struct Manifold {
    /// Body indices in the PhysicsWorld's body list.
    pub body_a: Entity,
    pub body_b: Entity,
    /// Collision normal pointing from A → B.
    pub normal: Float2,
    /// Penetration depth (positive = overlap).
    pub depth: f32,
    /// Contact point(s) in PhysicsWorld space.
    pub contacts: Vec<Float2>,
}
