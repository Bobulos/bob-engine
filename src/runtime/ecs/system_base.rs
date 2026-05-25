use crate::runtime::ecs::DynamicWorld;
use std::sync::Arc;

pub trait SystemBase: Send + Sync {
    fn on_start(&mut self, world: &Arc<DynamicWorld>);
    fn on_update(&mut self, world: &Arc<DynamicWorld>);
    fn on_destroy(&mut self, world: &Arc<DynamicWorld>);
}
