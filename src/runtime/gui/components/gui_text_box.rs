use fixed_str::FixedStr;



use crate::runtime::math::Float2;
use crate::runtime::rendering::gui_rendering::GuiCharInstance;
use crate::runtime::rendering::gui_rendering::PackedText;
use crate::runtime::rendering::gui_rendering::font_binder;
use crate::Component;
use crate::runtime::rendering::gui_rendering::font_binder::FontSheetBinding;

// 1 kb per text box
const MAX_TEXT_LENGTH: usize = 1024;

#[derive(Component!, Debug)]
pub struct GuiTextBox {
    pub dirty: bool,
    pub visible: bool,
    pub font_binding: FontSheetBinding,
    pub font_size: f32,
    pub used_length: usize,
    pub text: FixedStr<MAX_TEXT_LENGTH>,
    pub packed_text: PackedText<MAX_TEXT_LENGTH>,
}
impl GuiTextBox {
    pub fn new(text: String, font_binding: FontSheetBinding, font_size: f32) -> Self {
        debug_assert!(text.len() < MAX_TEXT_LENGTH, "text length must be less than {} characters", MAX_TEXT_LENGTH);
        let packed = font_binder::str_to_packed_text::<MAX_TEXT_LENGTH>(&text, font_binder::FontSheetBinding::NonStandardTestPallate);
        Self {
            dirty: true,
            visible: true,
            used_length: text.len() - 1,
            font_binding,
            font_size,
            text: FixedStr::from_slice(text.as_bytes()),
            packed_text: packed,
        }
    }
    pub fn generate_char_instances(&self, pos: Float2, size_x: f32) -> [GuiCharInstance; MAX_TEXT_LENGTH] {
        let mut instances = [GuiCharInstance::default(); MAX_TEXT_LENGTH];

        let uv_scale: [f32; 2] = font_binder::get_uv_scale(self.font_binding);
        let dimensions = font_binder::get_dimensions(self.font_binding);

        let half_f = self.font_size / 2.0;
        let l_start_x = pos.x + half_f;
        
        let mut y_cursor = pos.y + half_f;
        let mut x_cursor = l_start_x;
        for (i, c_packed) in self.packed_text.packed.iter().enumerate() {
            // line break
            if x_cursor >= size_x && self.text[i] as char == ' '{
                x_cursor = l_start_x;
                y_cursor += self.font_size;
                continue;
            }
            let w_pos = Float2::new(x_cursor, y_cursor);
            x_cursor += self.font_size;

            let uv_offset: [f32; 2] = font_binder::get_uv_offset(self.font_binding, c_packed, dimensions);
            
            instances[i] = GuiCharInstance { 
                position: w_pos.into(), 
                size: [self.font_size; 2],  
                uv_offset, 
                uv_scale 
            };
            
            // don't waste cycles on empty chars
            if i >= self.used_length {
                break;
            }
            // println!("{:?}", GuiCharInstance { 
            //     position: w_pos.into(), 
            //     size: [self.font_size; 2], 
            //     uv_offset, 
            //     uv_scale 
            // });
        }
        instances
    }
}
