use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::runtime::ecs::DynamicWorld;
use crate::runtime::scene::serialized_world::SerializedWorld;

type ComponentID = u64;
type SerializedWorldMap = HashMap<u64, HashMap<u64, serde_json::Value>>;
// pub fn dump_entitys(world: &Arc<DynamicWorld>) {
//     let lock = world.storages.read().unwrap();
// }

pub const SCENE_FILE_PATH: &'static str = "assets/scenes/saved/";
pub const SCENE_FILE_SUFFIX: &'static str = ".bscene";
pub fn create_scene_file_from_world(name: String, world: &Arc<DynamicWorld>) {
    let _ = fs::create_dir_all(SCENE_FILE_PATH);
    let formatted = format!(
        "{}{}{}",
        SCENE_FILE_PATH,
        name.to_string(),
        SCENE_FILE_SUFFIX
    );
    let serialized = serde_json::to_string_pretty(&serialize_world(world)).unwrap();
    fs::write(formatted, serialized);
}
fn serialize_world(world: &Arc<DynamicWorld>) -> SerializedWorld {
    let entity_count = *world.entities_count.read().unwrap();

    let mut component_defs: SerializedWorldMap = SerializedWorldMap::new();
    let lock = world.storages.read().unwrap();

    // todo!(
    //     "Make sure not to write to entitys that don't actually have the component like i'm doing now"
    // );

    for (component_id, store) in lock.iter() {
        let mut entity_store_count = 0;
        let read_store = store.read().unwrap();
        
        for e in 0..entity_count {
            if let Some(component_json) = read_store.serialize_component(e) {
                entity_store_count += 1;
                
                component_defs
                    .entry(*component_id)
                    .or_default()
                    .insert(e as u64, component_json);
            }
        }
        
        println!("cmpt id {} has {} entrys", component_id, entity_store_count);
    }

    SerializedWorld {
        entity_count: entity_count as u64,
        component_defs,
    }
}
