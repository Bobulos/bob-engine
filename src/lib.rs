mod runtime;
pub mod test;
pub use crate::app::App;
pub use stable_cmpt_id::StableID;
pub use winit::event_loop::EventLoop;
mod app;


// MARCOS
//pub trait Cmpt: Clone + Default + Copy + serde::Serialize + serde::Deserialize<'static> {}
pub trait StableTypeID {
    const ID: u64;
}

// All my nice aliasses
#[macro_use]
extern crate macro_rules_attribute;
derive_alias! {
    #[derive(Component!)] = #[derive(Clone, Default, stable_cmpt_id::StableID, Copy, serde::Serialize, serde::Deserialize)];
}
derive_alias! {
    #[derive(Serializable!)] = #[derive( serde::Serialize, serde::Deserialize)];
}
