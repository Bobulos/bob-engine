use crate::runtime::rendering::sprite_rendering::{SpriteSheetBinder, components::{Sprite, SpriteFrame, sprite_frame}};

/// Used inside sprite sheet binder
#[derive(Clone)]
pub struct SpriteAnimation {
    pub frame_abs: Vec<[f32; 2]>,
    pub frame_uv_bnd: Option<Vec<[f32; 2]>>,
}

impl SpriteAnimation {
    pub fn new(frame_abs: Vec<[f32; 2]>) -> Self {
        Self { 
            frame_abs, 
            frame_uv_bnd: None
        }
    }
    // /// Binds it to a sprite sheet binder
    // pub fn bind_animation_frame(&mut self, ) {
        
    // }
    // 
    /// Returns frame uv bnd frame
    pub fn get_frame(&self, frame: usize) -> [f32; 2] {
        if let Some(frame_uv_bnd) = &self.frame_uv_bnd {
            if let Some(&uv) = frame_uv_bnd.get(frame) {
                return uv;
            }
        }
        [0.0, 0.0]
    }
    pub fn go_to_next_frame(&self, sprite: &mut Sprite, sprite_frame: &mut SpriteFrame) {
        let mut next_frame = sprite_frame.frame + 1;
        //println!("sprite frame {} {}", sprite_frame.frame, self.frame_abs.len());
        if next_frame as usize >= self.frame_abs.len() {
            next_frame = 0;
        }
        sprite_frame.frame = next_frame;
        sprite.uv_offset = self.get_frame(next_frame as usize);
    }
}