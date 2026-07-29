// use sdl3::image::LoadTexture; // Trait for loading textures directly
// use sdl3::rect::Rect;
// use std::error::Error;
// use std::path::Path;

// mod runtime;
// pub mod test;
// // #[path = "engine/ecs/component_store.rs"]
// // pub mod component_store;
// // #[path = "engine/ecs/core_systems/core_components/mod.rs"]
// // pub mod core_components;
// // #[path = "engine/ecs/core_systems/mod.rs"]
// // pub mod core_systems;
// // #[path = "engine/math/float2.rs"]
// // pub mod float2;
// // #[path = "runtime/mod.rs"]
// // pub mod runtime;
// // #[path = "engine/rendering/tilemap/mod.rs"]
// // pub mod tilemap;
// use crate::app::App;
// pub use stable_cmpt_id::StableID;
// use winit::event_loop::EventLoop;
// mod app;
use bob_engine::{runtime::assets::AssetStore};
use winit::event_loop::EventLoop;
use bob_engine::app::App;
use bob_engine::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bob_engine running...");
    let event_loop = EventLoop::new()?;
    let mut app = App::default();

    let mut asset_store = AssetStore::new();
    include_asset!(&mut asset_store, "../assets/Tux.png");
    app.set_included_assets(asset_store);
    event_loop.run_app(&mut app)?; 
    Ok(())
}

// // MARCOS
// //pub trait Cmpt: Clone + Default + Copy + serde::Serialize + serde::Deserialize<'static> {}
// pub trait StableTypeID {
//     const ID: u64;
// }

// // All my nice aliasses
// #[macro_use]
// extern crate macro_rules_attribute;
// derive_alias! {
//     #[derive(Component!)] = #[derive(Clone, Default, stable_cmpt_id::StableID, Copy, serde::Serialize, serde::Deserialize)];
// }
// derive_alias! {
//     #[derive(Serializable!)] = #[derive( serde::Serialize, serde::Deserialize)];
// }
