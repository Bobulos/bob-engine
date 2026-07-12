use std::collections::HashSet;
use winit::{event::{ElementState, KeyEvent}, keyboard::PhysicalKey};
use crate::runtime::input::KeyCode;

pub struct Input {
    /// Keys currently held down
    held: HashSet<KeyCode>,
    /// Keys that went down this frame only
    just_pressed: HashSet<KeyCode>,
    /// Keys that were released this frame
    just_released: HashSet<KeyCode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    /// Call once at the START of each frame to clear per-frame state
    pub fn flush(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Call from App::window_event for every KeyboardInput event
    pub fn receive_input_from_app(&mut self, key_event: KeyEvent) {
        let key = key_event.physical_key;
        let code = KeyCode::from_winit(key).unwrap();
        match key_event.state {
            ElementState::Pressed => {
                // repeat=true means the key is being held, not freshly pressed
                if !key_event.repeat {
                    self.just_pressed.insert(code);
                }
                self.held.insert(code);
            }
            ElementState::Released => {
                self.held.remove(&code);
                self.just_released.insert(code);
            }
        }
    }

    /// True only on the frame the key was first pressed
    pub fn get_key_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    /// True every frame the key is held down
    pub fn get_key_down(&self, key: KeyCode) -> bool {
        self.held.contains(&key)
    }

    /// True only on the frame the key was released
    pub fn get_key_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}