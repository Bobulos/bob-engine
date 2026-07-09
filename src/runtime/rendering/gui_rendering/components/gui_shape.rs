use crate::Component;
use crate::runtime::assets::AssetHandle;

#[derive(Component!)]
pub struct GuiShape {
    pub visible: bool,

    // uv's
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}

impl GuiShape {
    pub fn new(
        asset_handle: AssetHandle,
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