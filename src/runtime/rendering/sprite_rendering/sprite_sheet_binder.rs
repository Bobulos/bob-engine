use crate::runtime::{math::Float2, rendering::sprite_rendering::{SpriteAnimation, components::{Sprite, SpriteFrame, sprite, sprite_frame}}};

// Forgiveness to correct bad clamping (expressed as a fraction of a single cell)
pub const BORDER_EDGE_WIDTH: f32 = 0.05;

#[derive(Clone)]
pub struct SpriteSheetBinder {
    pub size: [u16; 2],
    animation: Option<SpriteAnimation>,
    size_f: Float2,
    border_inset: [f32; 2],
}

impl SpriteSheetBinder {
    pub fn new(x: u16, y: u16, animation: Option<SpriteAnimation>) -> Self {
        let mut binder = Self {
            size: [x, y],
            animation,
            size_f: Float2::new(x as f32, y as f32),
            border_inset: [0.0, 0.0]
        };
        binder.border_inset =
            [
                (BORDER_EDGE_WIDTH / binder.size_f.x) * 0.5,
                (BORDER_EDGE_WIDTH / binder.size_f.y) * 0.5,
            ];
        binder.bind_animation();
        binder.clone()
    }

    pub fn bind_animation(&mut self) {
        if let Some(ref mut animation) = self.animation {
            // let cell_scale = get_bnd_scale_from_abs(self.size_f);
            let inset = self.border_inset;
            let mut bnd_frames = Vec::with_capacity(animation.frame_abs.len());
            for frame in animation.frame_abs.iter() {
                let offset = get_bnd_offset_from_abs_with_inset(*frame, self.size_f, inset);
                bnd_frames.push(offset);
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

    
    // #[inline]
    // fn border_inset(&self) -> [f32; 2] {
    //     [
    //         (BORDER_EDGE_WIDTH / self.size_f.x) * 0.5,
    //         (BORDER_EDGE_WIDTH / self.size_f.y) * 0.5,
    //     ]
    // }

    /// Use this if using animations because it sets the uv scale properly
    pub fn new_sprite_at_frame(&self, frame: [f32; 2]) -> Sprite {
        let inset = self.border_inset;
        Sprite::new(
            true,
            get_bnd_offset_from_abs_with_inset(frame, self.size_f, inset),
            get_bnd_scale_from_abs(self.size_f),
        )
    }

    /// given in x, y on the sprite sheet grid [0,0] is top left
    pub fn set_sprite_frame(&self, frame: [f32; 2], sprite: &mut Sprite) {
        let inset = self.border_inset;
        sprite.uv_offset = [
            (frame[0] / self.size_f.x) + inset[0],
            (frame[1] / self.size_f.y) + inset[1],
        ];
    }
}
pub fn get_bnd_offset_from_abs_with_inset(frame: [f32; 2], size_f: Float2, inset: [f32; 2]) -> [f32; 2] {
    [
        (frame[0] / size_f.x) + inset[0],
        (frame[1] / size_f.y) + inset[1],
    ] 
}
pub fn get_bnd_offset_from_abs_no_inset(frame: [f32; 2], size_f: Float2) -> [f32; 2] {
    [
        (frame[0] / size_f.x),
        (frame[1] / size_f.y),
    ] 
}
pub fn get_bnd_scale_from_abs(size_f: Float2) -> [f32; 2] {
    [
        (1.0 / size_f.x) * const { 1.0 - BORDER_EDGE_WIDTH },
        (1.0 / size_f.y) * const { 1.0 - BORDER_EDGE_WIDTH },
    ]
}