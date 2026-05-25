use crate::runtime::math::Float2;

pub struct PhysicsConfig {
    pub gravity: Float2,
    pub iterations: usize,
    pub slop: f32,
    pub correction_percent: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Float2::new(0.0, -9.81),
            iterations: 10,
            slop: 0.01,
            correction_percent: 0.4,
        }
    }
}
