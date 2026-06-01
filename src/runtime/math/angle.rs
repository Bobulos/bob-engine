use crate::runtime::math::Float2;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}
pub fn angle_to_point(a: Float2, b: Float2) -> f32 {
    let dif = a - b;
    let angle = dif.y.atan2(dif.x);
    angle
}
