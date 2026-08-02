pub const MAIN_WORLD: &str = "main";
pub const RENDER_GROUP: &str = "render_group";
pub const PHYSICS_GROUP: &str = "physics_group";
pub const PHYSICS_CONNECTION_GROUP: &str = "physics_connection_group";
pub const RENDER_BATCH_SIZE: usize = 1024 * 4; // 2^10
pub const FIXED_DT: f32 = 1.0 / 60.0; // 2^14