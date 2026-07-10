use std::collections::HashMap;
use crate::runtime::{ecs::DynamicWorld, scene::serialized_world::SerializedWorld};
type SerializedWorldMap = HashMap<u64, HashMap<u64, serde_json::Value>>;


fn load_scene_runtime() {
    
}
pub fn load_to_serialized_world() -> Option<SerializedWorld> {
    todo!("Figure out wheter to load embedded or not");
    None
}
pub fn load_serialized_world_to_dynamic_world() -> Option<DynamicWorld> {
    //world_serializer::SCENE_FILE_PATH


    None
}