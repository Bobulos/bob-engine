use crate::runtime::Engine;
use crate::runtime::assets::AssetStore;
use crate::runtime::rendering;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Fullscreen, Window, WindowAttributes};
use winit::event::ElementState;

pub static WINDOW_SIZE: (u32, u32) = (960, 540);
pub static FULLSCREEN: bool = false;
pub struct App {
    window: Option<Arc<Window>>,
    engine: Option<Engine>,
    included_assets: Option<AssetStore>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            engine: None,
            included_assets: None,
        }
    }
}
impl App {
    pub fn set_included_assets(&mut self, asset_store: AssetStore) {
        self.included_assets = Some(asset_store)
    }
    fn init_engine(&mut self) {
        let mut renderer = rendering::Renderer::new();
        pollster::block_on(renderer.init_window(Arc::clone(self.window.as_ref().expect("window not initialized when init_engine was called"))));

        let mut engine = Engine::new(renderer);
        engine.init(&mut self.included_assets.as_mut().unwrap()); // ThE big feller

        self.engine = Some(engine);
        println!("YO YO YO");
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Create window attributes required
            let mut attributes = WindowAttributes::default();
            attributes.title = "bob_engine".to_string();
            attributes.inner_size = Some(Size::new(Size::Physical(PhysicalSize::new(
                WINDOW_SIZE.0,
                WINDOW_SIZE.1,
            ))));

            if FULLSCREEN {
                attributes.fullscreen = Some(Fullscreen::Borderless(None));
            }

            let window = event_loop.create_window(attributes).unwrap();

            // let mut renderer = rendering::Renderer::new();
            // pollster::block_on(renderer.init_window(Arc::new(&window)));

            // let mut engine = Engine::new(renderer);
            // engine.init(); // Setup ECS, etc.

            self.window = Some(Arc::new(window));
            //self.engine = Some(engine);
            self.init_engine();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(engine) = &mut self.engine {
                    match state {
                        ElementState::Pressed => {
                            engine.input.write().unwrap().receive_mouse_button_pressed(button);
                        }
                        ElementState::Released => {
                            engine.input.write().unwrap().receive_mouse_button_released(button);
                        }
                    } 
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(engine) = &mut self.engine {
                    engine.input.write().unwrap().receive_mouse_moved(position);
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let Some(engine) = &mut self.engine {
                    engine.input.write().unwrap().receive_key_input_from_app(event);
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(engine) = &mut self.engine {
                    engine
                        .renderer
                        .write()
                        .unwrap()
                        .resize(physical_size.width, physical_size.height);

                    // I might actually not need this possimbly being called excessively
                    // Asumes that run doesn't catch it probably doesnt really matter too much.
                    engine.renderer.write().unwrap().update_camera();
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(engine) = &mut self.engine {
                    engine.run();
                }
                // Tell winit to keep redrawing as fast as possible (or on VSync)
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
