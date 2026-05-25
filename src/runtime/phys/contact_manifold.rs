use crate::runtime::math::Float2;

/// Up to 8 contact points with a count.
pub struct ContactManifold {
    pub points: [Float2; 8],
    pub count: usize,
}

impl ContactManifold {
    pub fn new() -> Self {
        Self {
            points: [Float2::ZERO; 8],
            count: 0,
        }
    }

    pub fn push(&mut self, p: Float2) {
        if self.count < 8 {
            self.points[self.count] = p;
            self.count += 1;
        }
    }
}
