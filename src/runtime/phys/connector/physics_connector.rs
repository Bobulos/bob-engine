use crate::runtime::{ecs::Entity, math::Float2};

#[derive(Debug, Clone, Copy)]
pub struct PhysicsConnector {
    pub is_intact: bool,
    pub connection_strength_ln_sq: f32,
    pub connection_strength_ang: f32,
    pub connections: [Option<Entity>; 4],
    pub anchors: [Option<Float2>; 4],
}

impl PhysicsConnector {
    pub fn new(
        connection_strength_ln: f32,
        connection_strength_ang: f32,
        connections: [Option<Entity>; 4],
        anchors: [Option<Float2>; 4],
    ) -> Self {
        Self {
            is_intact: true,
            connection_strength_ln_sq: connection_strength_ln * connection_strength_ln,
            connection_strength_ang,
            connections,
            anchors,
        }
    }
}
