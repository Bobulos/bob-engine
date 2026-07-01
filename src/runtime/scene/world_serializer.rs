use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{any::Any, sync::Arc};

use crate::runtime::ecs::DynamicWorld;
use crate::runtime::rendering::sprite_rendering::components;
use crate::{
    Component, StableTypeID,
    runtime::ecs::{Entity, component_store::ComponentStore},
};

pub struct WorldSerializer {}

impl WorldSerializer {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn dump_entitys(world: &Arc<DynamicWorld>) {
    let entity_count = *world.entities_count.read().unwrap();
    let lock = world.storages.read().unwrap();
    for e in 0..entity_count {
        for store in lock.iter() {
            let yert = store.1.read().unwrap().serialize_component(e);
            println!("{}", yert);
        }
    }
}
