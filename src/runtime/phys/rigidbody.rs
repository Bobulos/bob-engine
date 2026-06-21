use crate::runtime::math::Float2;
use crate::runtime::phys::Aabb;
use crate::runtime::phys::Shape;

#[derive(Debug, Clone)]
pub struct PhysicsTransform {
    pub position: Float2,
    pub rotation: f32, // radians
}

#[derive(Debug, Clone)]
pub struct PhysicsVelocity {
    pub velocity: Float2,
    pub angular_velocity: f32, // radians / s
}
#[derive(Debug, Clone)]
pub struct PhysicsMass {
    pub inv_mass: f32,
    pub inv_inertia: f32,
}

#[derive(Debug, Clone)]
pub struct PhysicsForce {
    pub force: Float2,
    pub torque: f32,
}
#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    pub restitution: f32, // 0 = perfectly inelastic, 1 = perfectly elastic
    pub friction: f32,    // Coulomb friction coefficient
}
#[derive(Debug, Clone)]
pub struct PhysicsFlags {
    pub is_static: bool,
}
#[derive(Debug, Clone)]
pub struct PhysicsShape {
    pub shape: Shape,
}

pub fn apply_impulse(
    physics_mass: &PhysicsMass,
    physics_velocity: &mut PhysicsVelocity,
    impulse: Float2,
    r: Float2,
) {
    physics_velocity.velocity += impulse * physics_mass.inv_mass;
    physics_velocity.angular_velocity += r.cross(impulse) * physics_mass.inv_inertia;
}

/// Velocity of a point fixed to this body at PhysicsWorld-space offset `r`.
pub fn velocity_at(physics_velocity: &PhysicsVelocity, r: Float2) -> Float2 {
    physics_velocity.velocity + Float2::cross_scalar_vec(physics_velocity.angular_velocity, r)
}

pub fn aabb(physics_shape: &PhysicsShape, physics_transform: &PhysicsTransform) -> Aabb {
    physics_shape
        .shape
        .aabb(physics_transform.position, physics_transform.rotation)
}

pub fn integrate(&mut self, dt: f32, gravity: Float2) {
    // Add if i need later

    // if self.is_static {
    //     return;
    // }

    // Semi-implicit Euler integration
    let accel = self.force * self.inv_mass + gravity;
    self.velocity += accel * dt;
    self.position += self.velocity * dt;

    let alpha = self.torque * self.inv_inertia;
    self.angular_velocity += alpha * dt;
    self.rotation += self.angular_velocity * dt;

    // Reset accumulators
    self.force = Float2::ZERO;
    self.torque = 0.0;
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    // Pose
    pub position: Float2,
    pub rotation: f32, // radians

    // Velocity
    pub velocity: Float2,
    pub angular_velocity: f32, // radians / s

    // Mass properties
    /// Inverse mass (0 = static / infinite mass).
    pub inv_mass: f32,
    /// Inverse moment of inertia (0 = static).
    pub inv_inertia: f32,

    // Material
    pub restitution: f32, // 0 = perfectly inelastic, 1 = perfectly elastic
    pub friction: f32,    // Coulomb friction coefficient

    // Shape
    pub shape: Shape,

    // Accumulated forces (reset each frame)
    pub force: Float2,
    pub torque: f32,

    // Flags
    pub is_static: bool,
}

impl RigidBody {
    pub fn new(shape: Shape, mass: f32, pos: Float2, rotation: f32) -> Self {
        let (inv_mass, inv_inertia) = if mass <= 0.0 {
            (0.0, 0.0)
        } else {
            let i = mass * shape.inertia_factor();
            (1.0 / mass, 1.0 / i)
        };
        Self {
            position: pos,
            rotation: rotation,
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

    pub fn new_static(shape: Shape, pos: Float2, rotation: f32) -> Self {
        let mut b = Self::new(shape, 0.0, pos, rotation);
        b.is_static = true;
        b
    }

    pub fn apply_force(&mut self, f: Float2) {
        self.force += f;
    }

    pub fn apply_force_at(&mut self, f: Float2, physics_world_point: Float2) {
        self.force += f;
        let r = physics_world_point - self.position;
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
        self.shape.aabb(self.position, self.rotation)
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
        self.rotation += self.angular_velocity * dt;

        // Reset accumulators
        self.force = Float2::ZERO;
        self.torque = 0.0;
    }
}
