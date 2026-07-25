use crate::runtime::math::Float2;

pub fn random_float2(min: Float2, max: Float2) -> Float2 {
    Float2 {
        x: rand::random_range(min.x..max.x),
        y: rand::random_range(min.y..max.y),
    }
}