use crate::runtime::rendering::VertexLayout;
use wgpu::VertexFormat::*;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GuiInstance {
    pub position:     [f32; 2],  // location 2
    pub size:         [f32; 2],  // location 3
    pub uv_offset:    [f32; 2],  // location 4
    pub uv_scale:     [f32; 2],  // location 5
    pub corner_radius: f32,      // location 6
    pub color:        [f32; 4],  // location 7
    pub border_width: f32,       // location 8
    pub border_color: [f32; 4],  // location 9
    pub _pad:         [u32; 3],  // realign to 16 bytes
}

impl VertexLayout for GuiInstance {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<GuiInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { shader_location: 2, offset: 0,  format: Float32x2 }, // position
                wgpu::VertexAttribute { shader_location: 3, offset: 8,  format: Float32x2 }, // size
                wgpu::VertexAttribute { shader_location: 4, offset: 16, format: Float32x2 }, // uv_offset
                wgpu::VertexAttribute { shader_location: 5, offset: 24, format: Float32x2 }, // uv_scale
                wgpu::VertexAttribute { shader_location: 6, offset: 32, format: Float32 }, // corner_radius
                wgpu::VertexAttribute { shader_location: 7, offset: 36, format: Float32x4 }, // color
                wgpu::VertexAttribute { shader_location: 8, offset: 52, format: Float32 }, // border_width
                wgpu::VertexAttribute { shader_location: 9, offset: 56, format: Float32x4 }, // border_color
                
            ],
        }
    }
}
impl Default for GuiInstance {
    fn default() -> Self {
        Self { 
            position: [f32::MIN, f32::MIN], 
            size: [100.0,100.0], 
            uv_offset: [0.0,0.0], 
            uv_scale: [1.0,1.0], 
            corner_radius: 1.0, 
            color: [1.0, 0.0, 0.0, 1.0], 
            border_width: 0.3, 
            border_color: [1.0,1.0,1.0,1.0], 
            _pad: [0; 3] 
        }
    }
}