use u4::U4x2;
use crate::runtime::rendering::gui_rendering::font_binder;
use fixed_str::FixedStr;
// 1 kb per text box
const MAX_TEXT_LENGTH: usize = 1024;

pub struct GuiTextBox {
    pub text: FixedStr<MAX_TEXT_LENGTH>,
    pub packed_text: [U4x2; MAX_TEXT_LENGTH],
}
impl GuiTextBox {
    pub fn new(text: String) -> Self {
        debug_assert!(text.len() < MAX_TEXT_LENGTH, "text length must be less than {} characters", MAX_TEXT_LENGTH);

        let mut packed_text = [U4x2::default(); MAX_TEXT_LENGTH];
        for i in packed_text.iter_mut() {
            i = crate::runtime::rendering::gui_rendering::font_binder::
        }
        
        Self {
            text: FixedStr::from_slice(text.as_bytes()),
            packed_text: [U4x2::default(); MAX_TEXT_LENGTH],
        }
    }
    
}