use std::usize;
use u4::U4x2;
use fixed_str::FixedStr;
use serde_with;

use crate::runtime::rendering::gui_rendering::PackedText;
use crate::runtime::rendering::gui_rendering::font_binder;
use crate::Component;

// 1 kb per text box
const MAX_TEXT_LENGTH: usize = 1024;

#[derive(Component!)]
pub struct GuiTextBox {
    pub dirty: bool,
    pub visible: bool,
    pub text: FixedStr<MAX_TEXT_LENGTH>,
    pub packed_text: PackedText<MAX_TEXT_LENGTH>,
}
impl GuiTextBox {
    pub fn new(text: String) -> Self {
        debug_assert!(text.len() < MAX_TEXT_LENGTH, "text length must be less than {} characters", MAX_TEXT_LENGTH);
        let packed = font_binder::str_to_packed_text::<MAX_TEXT_LENGTH>(&text, font_binder::FontSheetBinding::NonStandardTestPallate);
        Self {
            dirty: true,
            visible: true,
            text: FixedStr::from_slice(text.as_bytes()),
            packed_text: packed,
        }
    }
    
}
