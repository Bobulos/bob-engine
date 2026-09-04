use std::{any::Any, sync};
use crate::StableTypeID;
use serde::{Deserialize, Serialize};

type ComponentID = u64;
pub struct SingletonStore<T: StableTypeID
        + Default
        + Any
        + Send
        + Sync
        + Copy
        + Clone
        + Serialize
        + Deserialize<'static>
        + 'static,
> {
    component_ids: ComponentID,

    /// None if dead
    component: Option<T>,
}

impl<T: StableTypeID
        + Default
        + Any
        + Send
        + Sync
        + Copy
        + Clone
        + Serialize
        + Deserialize<'static>
        + 'static,
> SingletonStore<T> {
    pub fn new(component_id: ComponentID, component: T) -> Self {
        Self {
            component_ids: component_id,
            component: Some(component),
        }
    }

    pub fn set(&mut self, component: Option<T>) {
        self.component = component;
    }
    pub fn get(&self) -> Option<&T> {
        &self.component
    }
    pub fn get_mut(&mut self) -> Option<&m>
    // pub fn set(&mut self, component_id: ComponentID, component: T) {
        
    // }
    // pub fn get(&self, component_id: ComponentID, component: T) {
        
    // }
}

impl<T: StableTypeID
        + Default
        + Any
        + Send
        + Sync
        + Copy
        + Clone
        + Serialize
        + Deserialize<'static>
        + 'static,
> AnySingletonStore for SingletonStore<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn insert_default(&mut self, entity: usize) {
        self.component = T::default();
    }

    fn serialize(&self) -> Option<serde_json::Value> {
        serde_json::to_value(self.component).ok()
    }
}

pub trait AnySingletonStore: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    
    fn insert_default(&mut self, entity: usize);

    fn as_any_mut(&mut self) -> &mut dyn Any;

    // should go in the header of the ecs thing 
    fn serialize(&self) -> Option<serde_json::Value>;
}