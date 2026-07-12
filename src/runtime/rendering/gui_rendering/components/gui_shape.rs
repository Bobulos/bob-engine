use std::default::Default;
use serde;
use crate::{Component, runtime::rendering::Color};

#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
pub enum Border {
    #[default]
    Borderless,
    /// Color Width Radius
    Bordered(Color, f32, f32)
}
// impl Default for Border {
//     fn default() -> Self {
//         Self::Borderless
//     }
// }
#[derive(Component!)]
pub struct GuiShape {
    pub visible: bool,
    pub dirty: bool,
    pub size: [f32; 2],
    pub fill_color: Color,

    pub border: Border,
    // pub border_color: Color,
    // pub border_width: f32,
    // pub corner_radius: f32,
    // uv's
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}

impl GuiShape {
    pub fn new(
        visible: bool,
        size: [f32; 2],
        fill_color: Color,
        border: Border,
        // border_color: Color,
        // border_width: f32,
        // corner_radius: f32,
        uv_offset: [f32; 2],
        uv_scale: [f32; 2],
    ) -> Self {
        Self {
            visible,
            uv_offset,
            uv_scale,
            border,
            size,
            fill_color,
            dirty: true,
            // border_color,
            // corner_radius,
        }
    }
}