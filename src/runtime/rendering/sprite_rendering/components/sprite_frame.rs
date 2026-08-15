pub struct SpriteFrame {
    /// indexes into a sprite animation, stored inside a sytem SpriteSheetBinder
    pub frame: u16,
}
impl SpriteFrame {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}