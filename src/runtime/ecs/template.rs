use std::any::Any;

use crate::runtime::ecs::DynamicWorld;

pub trait Template: Send + Sync {
    fn instantiate(&self, world: &Arc<DynamicWorld>, overrides: &(dyn Any + Send + Sync));
}