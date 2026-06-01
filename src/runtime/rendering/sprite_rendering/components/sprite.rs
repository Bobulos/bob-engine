#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub batch_index: usize, // Index into Renderer.batches
    pub index: usize,       // Index into the batch
    //pub enabled: bool,          // Whether this sprite should be rendered
    pub atlas_id: usize, // ID to look up in TextureCache
    pub width: u32,
    pub height: u32,
    pub visible: bool,

    // uv's
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}

impl Sprite {
    pub fn new(
        atlas_id: usize,
        width: u32,
        height: u32,
        visible: bool,
        uv_offset: [f32; 2],
        uv_scale: [f32; 2],
    ) -> Self {
        Self {
            batch_index: 0, // Will be set later when the sprite is added to a batch
            index: usize::MAX,
            //enabled: true,
            atlas_id,
            width,
            height,
            visible,
            uv_offset,
            uv_scale,
        }
    }
}
