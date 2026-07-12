use std::default::Default;
use winit::event;
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    #[default]
    None,
}

impl MouseButton {
    pub fn from_winit(button: winit::event::MouseButton) -> MouseButton {
        match button {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Right => MouseButton::Right,
            _ => MouseButton::None,
        }
    }
}