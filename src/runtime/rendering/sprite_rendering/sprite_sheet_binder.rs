use crate::runtime::{math::Float2, rendering::sprite_rendering::{SpriteAnimation, components::{Sprite, SpriteFrame, sprite, sprite_frame}}};

/// This lil guy drives sprite sheet animations from a frame number calculating uv's
#[derive(Clone)]
pub struct SpriteSheetBinder {
    pub size: [u16; 2],
    animation: Option<SpriteAnimation>,  
    size_f: Float2,
}
impl SpriteSheetBinder {
    /// Size of the sprite sheet in frame by frame
    pub fn new(x: u16, y: u16, animation: Option<SpriteAnimation>) -> Self {
        let mut binder = Self { 
            size: [x, y], 
            animation,
            size_f: Float2::new(x as f32, y as f32)
        };
        binder.bind_animation();
        binder.clone()
    }
    pub fn bind_animation(&mut self) {
        if let Some(ref mut animation) = self.animation {
            let mut bnd_frames = Vec::with_capacity(animation.frame_abs.len());
            for (i, f) in animation.frame_abs.iter().enumerate() {
                bnd_frames[i] = get_bnd_frame_from_abs(self.size_f, f);
            }
            animation.frame_uv_bnd = Some(bnd_frames);
        }
    }
    pub fn start_animation(&mut self, sprite: &mut Sprite, sprite_frame: &mut SpriteFrame) {
        if let Some(ref mut animation) = self.animation {
            sprite_frame.frame = 0;
            let frame = animation.get_frame(0);
            sprite.uv_offset = frame;
        }
    }
    pub fn run_animation(&self, sprite: &mut Sprite, sprite_frame: &mut SpriteFrame) {
        if let Some(ref animation) = self.animation {
            animation.go_to_next_frame(sprite, sprite_frame);
        }
    }
    //
    // These guys don't need to have a sprite frame
    //
    /// Use this if using animations because it sets the uv scale propperly
    pub fn new_sprite_at_frame(&self, frame: [f32; 2]) -> Sprite {
        Sprite::new(true, [frame[0] / self.size_f.x, frame[1] / self.size_f.y ], [1.0 / self.size_f.x, 1.0 / self.size_f.y])
    }
    /// given in x, y on the sprite sheet grid [0,0] is top left
    pub fn set_sprite_frame(&self, frame: [f32; 2], sprite: &mut Sprite) {
        // unnesecary once set add a construct with frame to fix later
        sprite.uv_offset = [frame[0] / self.size_f.x, frame[1] / self.size_f.y ];
        // let frame_f = frame as f32;
        
        // let frame_x = frame_f / self.size.x;
        // // normalize to 0 - 1
        // let frame_x_f = frame_x / 
    }
}
pub fn get_bnd_frame_from_abs(size_f: Float2, frame: &[f32; 2]) -> [f32; 2] {
    [frame[0] / size_f.x, frame[1] / size_f.y ]
}