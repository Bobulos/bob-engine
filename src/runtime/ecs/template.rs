use std::any::Any;

pub trait Template: Send + Sync {
    fn instantiate(&self, overrides: &(dyn Any + Send + Sync));
}