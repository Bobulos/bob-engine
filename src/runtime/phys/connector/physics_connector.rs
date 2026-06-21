use crate::runtime::{ecs::Entity, math::Float2};

#[derive(Debug, Clone, Copy)]
pub struct PhysicsConnection {
    pub cxn: Entity,
    pub anc: Float2,
}
impl PhysicsConnection {
    pub fn new(cxn: Entity, anc: Float2) -> Self {
        Self { cxn, anc }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsConnector {
    pub is_intact: bool,
    pub cxn_strength_ln_sq: f32,
    pub cxn_strength_ang: f32,
    pub cxns: [Option<PhysicsConnection>; 4],
}

impl PhysicsConnector {
    pub fn new(
        connection_strength_ln: f32,
        cxn_strength_ang: f32,
        cxns: [Option<PhysicsConnection>; 4],
    ) -> Self {
        Self {
            is_intact: true,
            cxn_strength_ln_sq: connection_strength_ln * connection_strength_ln,
            cxn_strength_ang,
            cxns,
        }
    }
}
