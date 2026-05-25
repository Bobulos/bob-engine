// use sdl3::image::LoadTexture; // Trait for loading textures directly
// use sdl3::rect::Rect;
// use std::error::Error;
// use std::path::Path;

mod runtime;
pub mod test;
// #[path = "engine/ecs/component_store.rs"]
// pub mod component_store;
// #[path = "engine/ecs/core_systems/core_components/mod.rs"]
// pub mod core_components;
// #[path = "engine/ecs/core_systems/mod.rs"]
// pub mod core_systems;
// #[path = "engine/math/float2.rs"]
// pub mod float2;
// #[path = "runtime/mod.rs"]
// pub mod runtime;
// #[path = "engine/rendering/tilemap/mod.rs"]
// pub mod tilemap;

use crate::app::App;
use winit::event_loop::EventLoop;
mod app;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bob_engine running...");
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
