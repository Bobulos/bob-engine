use crate::b_engine::physics_systems::Aabb;
use crate::b_engine::physics_systems::Shape;
use crate::float2::Float2;

#[derive(Debug, Clone)]
pub struct RigidBody {
    // ── Pose ──────────────────────────────────────────────────────────────────
    pub position: Float2,
    pub angle: f32, // radians

    // ── Velocity ──────────────────────────────────────────────────────────────
    pub velocity: Float2,
    pub angular_velocity: f32, // radians / s

    // ── Mass properties ───────────────────────────────────────────────────────
    /// Inverse mass (0 = static / infinite mass).
    pub inv_mass: f32,
    /// Inverse moment of inertia (0 = static).
    pub inv_inertia: f32,

    // ── Material ──────────────────────────────────────────────────────────────
    pub restitution: f32, // 0 = perfectly inelastic, 1 = perfectly elastic
    pub friction: f32,    // Coulomb friction coefficient

    // ── Shape ─────────────────────────────────────────────────────────────────
    pub shape: Shape,

    // ── Accumulated forces (reset each frame) ─────────────────────────────────
    force: Float2,
    torque: f32,

    // ── Flags ─────────────────────────────────────────────────────────────────
    pub is_static: bool,
}

impl RigidBody {
    pub fn new(shape: Shape, mass: f32, pos: Float2) -> Self {
        let (inv_mass, inv_inertia) = if mass <= 0.0 {
            (0.0, 0.0)
        } else {
            let i = mass * shape.inertia_factor();
            (1.0 / mass, 1.0 / i)
        };
        Self {
            position: pos,
            angle: 0.0,
            velocity: Float2::ZERO,
            angular_velocity: 0.0,
            inv_mass,
            inv_inertia,
            restitution: 0.4,
            friction: 0.3,
            shape,
            force: Float2::ZERO,
            torque: 0.0,
            is_static: mass <= 0.0,
        }
    }

    /// Convenience: create a static (immovable) body.
    pub fn new_static(shape: Shape, pos: Float2) -> Self {
        let mut b = Self::new(shape, 0.0, pos);
        b.is_static = true;
        b
    }

    pub fn apply_force(&mut self, f: Float2) {
        self.force += f;
    }

    pub fn apply_force_at(&mut self, f: Float2, PhysicsWorld_point: Float2) {
        self.force += f;
        let r = PhysicsWorld_point - self.position;
        self.torque += r.cross(f);
    }

    pub fn apply_impulse(&mut self, impulse: Float2, r: Float2) {
        self.velocity += impulse * self.inv_mass;
        self.angular_velocity += r.cross(impulse) * self.inv_inertia;
    }

    /// Velocity of a point fixed to this body at PhysicsWorld-space offset `r`.
    pub fn velocity_at(&self, r: Float2) -> Float2 {
        self.velocity + Float2::cross_scalar_vec(self.angular_velocity, r)
    }

    pub fn aabb(&self) -> Aabb {
        self.shape.aabb(self.position, self.angle)
    }

    pub fn integrate(&mut self, dt: f32, gravity: Float2) {
        if self.is_static {
            return;
        }

        // Semi-implicit Euler integration
        let accel = self.force * self.inv_mass + gravity;
        self.velocity += accel * dt;
        self.position += self.velocity * dt;

        let alpha = self.torque * self.inv_inertia;
        self.angular_velocity += alpha * dt;
        self.angle += self.angular_velocity * dt;

        // Reset accumulators
        self.force = Float2::ZERO;
        self.torque = 0.0;
    }
}
