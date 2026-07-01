use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{any::Any, sync::Arc};

use crate::runtime::ecs::DynamicWorld;
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

fn serialize_component<T: Default + Serialize + Deserialize<'static> + 'static>(
    world: Arc<DynamicWorld>,
    store: ComponentStore<T>,
    entity: Entity,
) {
    world.
    let c = store.get(entity.0);
    let pretty_json = serde_json::to_string_pretty(&c.unwrap());
    println!("Pretty:\n{}", pretty_json.unwrap());
}
