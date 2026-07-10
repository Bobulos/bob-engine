use crate::Component;


#[derive(Component!)]
pub struct Sprite {
    pub visible: bool,

    // uv's
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}

impl Sprite {
    pub fn new(
        visible: bool,
        uv_offset: [f32; 2],
        uv_scale: [f32; 2],
    ) -> Self {
        Self {
            visible,
            uv_offset,
            uv_scale,
        }
    }
}
