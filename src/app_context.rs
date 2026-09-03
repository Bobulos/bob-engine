#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    pub window_size: (u32, u32),
    pub full_screen: bool,
}

impl AppContext {
    pub fn new(window_size: (u32, u32), full_screen: bool) -> Self {
        Self {
            window_size,
            full_screen,
        }
    }
    pub fn default() -> Self {
        Self {
            window_size: (800, 600),
            full_screen: false,
        }
    }
}