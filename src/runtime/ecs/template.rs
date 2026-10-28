use std::any::Any;
use std::sync::Arc;

use crate::runtime::ecs::DynamicWorld;
type Overide = dyn Any + Send + Sync;
pub trait Template: Send + Sync {
    fn instantiate(&self, world: &Arc<DynamicWorld>, overrides: &Overide);
}