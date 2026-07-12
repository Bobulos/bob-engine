use std::collections::HashSet;
use winit::{dpi::PhysicalPosition, event::{ElementState, KeyEvent, WindowEvent}, keyboard::PhysicalKey};
use crate::runtime::input::{KeyCode, mouse_button::MouseButton};

/// Mouse x y int px
pub struct MousePosition(u32, u32);

pub struct Input {
    /// Mouse position
    mouse_position: MousePosition,

    /// Mouse button held down
    mouse_held: HashSet<MouseButton>,
    /// Mouse that went down this frame only
    mouse_just_pressed: HashSet<MouseButton>,
    /// Mouse that were released this frame
    mouse_just_released: HashSet<MouseButton>,

    /// Keys currently held down
    key_held: HashSet<KeyCode>,
    /// Keys that went down this frame only
    key_just_pressed: HashSet<KeyCode>,
    /// Keys that were released this frame
    key_just_released: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_position: MousePosition(0, 0),
            mouse_held: HashSet::new(),
            mouse_just_pressed: HashSet::new(),
            mouse_just_released: HashSet::new(),
            key_held: HashSet::new(),
            key_just_pressed: HashSet::new(),
            key_just_released: HashSet::new(),
        }
    }
    pub fn mouse_position(&self) -> (u32, u32) {
        (self.mouse_position.0, self.mouse_position.1)
    }
    /// Call once at the START of each frame to clear per-frame state
    pub fn flush(&mut self) {
        self.key_just_pressed.clear();
        self.key_just_released.clear();
        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();
    }
    pub fn receive_mouse_button_pressed(&mut self, button: winit::event::MouseButton) {
        let button = MouseButton::from_winit(button);
        self.mouse_just_pressed.insert(button);
        self.mouse_held.insert(button);
    }
    
    pub fn receive_mouse_button_released(&mut self, button: winit::event::MouseButton) {
        let button = MouseButton::from_winit(button);
        self.mouse_held.remove(&button);
        self.mouse_just_released.insert(button);
    }
    /// Call from App::window_event for every MouseInput event
    pub fn receive_mouse_moved(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = MousePosition(position.x as u32, position.y as u32);
    }

    /// Call from App::window_event for every KeyboardInput event
    pub fn receive_key_input_from_app(&mut self, key_event: KeyEvent) {
        let key = key_event.physical_key;
        let code = KeyCode::from_winit(key).unwrap();
        match key_event.state {
            ElementState::Pressed => {
                // repeat=true means the key is being held, not freshly pressed
                if !key_event.repeat {
                    self.key_just_pressed.insert(code);
                }
                self.key_held.insert(code);
            }
            ElementState::Released => {
                self.key_held.remove(&code);
                self.key_just_released.insert(code);
            }
        }
    }

    /// True only on the frame the key was first pressed
    pub fn get_key_pressed(&self, key: KeyCode) -> bool {
        self.key_just_pressed.contains(&key)
    }

    /// True every frame the key is held down
    pub fn get_key_down(&self, key: KeyCode) -> bool {
        self.key_held.contains(&key)
    }

    /// True only on the frame the key was released
    pub fn get_key_released(&self, key: KeyCode) -> bool {
        self.key_just_released.contains(&key)
    }

    
    /// true only on the frame the button was first pressed
    pub fn get_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_just_pressed.contains(&button)
    }

    /// true every frame the key is held down
    pub fn get_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_held.contains(&button)
    }

    /// True only on the frame the key was released
    pub fn get_mouse_button_released(&self, button: MouseButton) -> bool {
        self.mouse_just_released.contains(&button)
    }
}