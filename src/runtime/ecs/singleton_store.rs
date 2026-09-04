use std::{any::Any, sync};
pub struct SingletonStore {
    
}

pub trait AnySingletonStore: Any + Send + Sync {
    
}