use crate::Component;
#[derive(Component!)]

pub struct SpriteFrame {
    /// indexes into a sprite animation, stored inside a sytem SpriteSheetBinder
    pub frame: u16,
}
impl SpriteFrame {
    pub fn new(frame: u16) -> Self {
        Self { frame }
    }
}