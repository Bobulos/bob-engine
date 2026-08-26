use u4::U4x2;
use crate::runtime::rendering::gui_rendering::font_binder;

// 1 kb per text box
const MAX_TEXT_LENGTH: usize = 1024;

pub struct GuiTextBox {
    pub text: String,
    pub packed_text: [U4x2; MAX_TEXT_LENGTH],
}
impl GuiTextBox {
    pub fn new(text: String) -> Self {
        debug_assert!(text.len() < MAX_TEXT_LENGTH, "text length must be less than {} characters", MAX_TEXT_LENGTH);

        let mut packed_text = [U4x2::default(); MAX_TEXT_LENGTH];
        
        Self {
            text,
            packed_text: [U4x2::default(); MAX_TEXT_LENGTH],
        }
    }
    
}