use crate::runtime::ecs::{SystemBase, DynamicWorld};
use std::sync::Arc;

pub struct AudioSystem {
}
impl SystemBase for AudioSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {
        
    }
    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        
    } 
    fn on_destroy(&mut self, world: &Arc<DynamicWorld>) {
        
    }
}