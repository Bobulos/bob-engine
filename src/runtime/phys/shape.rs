use crate::runtime::phys::aabb::Aabb;
use crate::float2::Float2;
#[derive(Debug, Clone)]
pub enum Shape {
    Circle { radius: f32 },
    Rect { half_w: f32, half_h: f32 },
}

impl Shape {
    /// Moment of inertia for a unit mass. Caller scales by actual mass.
    pub fn inertia_factor(&self) -> f32 {
        match self {
            Shape::Circle { radius } => 0.5 * radius * radius,
            Shape::Rect { half_w, half_h } => {
                (4.0 * half_w * half_w + 4.0 * half_h * half_h) / 12.0
            }
        }
    }

    pub fn aabb(&self, pos: Float2, angle: f32) -> Aabb {
        match self {
            Shape::Circle { radius } => Aabb {
                min: Float2::new(pos.x - radius, pos.y - radius),
                max: Float2::new(pos.x + radius, pos.y + radius),
            },
            Shape::Rect { half_w, half_h } => {
                // Compute PhysicsWorld-space corners and take extremes.
                let corners = [
                    Float2::new(*half_w, *half_h),
                    Float2::new(-half_w, *half_h),
                    Float2::new(-half_w, -*half_h),
                    Float2::new(*half_w, -*half_h),
                ];
                let mut min = Float2::new(f32::MAX, f32::MAX);
                let mut max = Float2::new(f32::MIN, f32::MIN);
                for c in &corners {
                    let w = c.rotate(angle) + pos;
                    min.x = min.x.min(w.x);
                    min.y = min.y.min(w.y);
                    max.x = max.x.max(w.x);
                    max.y = max.y.max(w.y);
                }
                Aabb { min, max }
            }
        }
    }

    /// PhysicsWorld-space vertices of a rectangle (empty for circles).
    pub fn rect_vertices(&self, pos: Float2, angle: f32) -> Vec<Float2> {
        match self {
            Shape::Rect { half_w, half_h } => vec![
                Float2::new(*half_w, *half_h).rotate(angle) + pos,
                Float2::new(-half_w, *half_h).rotate(angle) + pos,
                Float2::new(-half_w, -*half_h).rotate(angle) + pos,
                Float2::new(*half_w, -*half_h).rotate(angle) + pos,
            ],
            Shape::Circle { .. } => vec![],
        }
    }
}
