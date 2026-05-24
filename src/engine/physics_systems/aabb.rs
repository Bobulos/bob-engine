use crate::float2::Float2;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Float2,
    pub max: Float2,
}

impl Aabb {
    pub fn overlaps(self, other: Self) -> bool {
        self.max.x >= other.min.x
            && self.min.x <= other.max.x
            && self.max.y >= other.min.y
            && self.min.y <= other.max.y
    }
}
