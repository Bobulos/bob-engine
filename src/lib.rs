pub mod runtime;
pub mod test;
pub mod app;
pub mod constants;
pub mod app_context;

pub use crate::app_context::AppContext;
pub use serde;
pub use component::component;
pub use component::StableID;
pub use crate::app::App;
pub use winit::event_loop::EventLoop;

// now in partent
// MARCOS
//pub trait Cmpt: Clone + Default + Copy + serde::Serialize + serde::Deserialize<'static> {}
pub trait StableTypeID {
    const ID: u64;
}

// All my nice aliasses
#[macro_use]
extern crate macro_rules_attribute;

derive_alias! {
    #[derive(Component!)] = #[derive(Clone, Default, component::StableID, Copy, serde::Serialize, serde::Deserialize)];
}

derive_alias! {
    #[derive(Serializable!)] = #[derive(serde::Serialize, serde::Deserialize)];
}