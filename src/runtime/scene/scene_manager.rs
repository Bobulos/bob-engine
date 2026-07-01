use crate::runtime::scene::world_serializer::WorldSerializer;

pub struct SceneManager {
    
    world_serializer: WorldSerializer,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            world_serializer: WorldSerializer::new(),
        }
    }
}
