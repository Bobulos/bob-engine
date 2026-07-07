use crate::runtime::rendering::VertexLayout;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GuiInstance {
    pub position:   [f32; 2],
    pub size:       [f32; 2],
    pub uv_offset:  [f32; 2],
    pub uv_scale:   [f32; 2],
    pub rotation:   f32,
    pub color:      [f32; 4],
    pub _pad:       [f32; 3], // align to 16 bytes
}

impl VertexLayout for GuiInstance {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<GuiInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { shader_location: 2, offset: 0,  format: wgpu::VertexFormat::Float32x2 }, // position
                wgpu::VertexAttribute { shader_location: 3, offset: 8,  format: wgpu::VertexFormat::Float32x2 }, // size
                wgpu::VertexAttribute { shader_location: 4, offset: 16, format: wgpu::VertexFormat::Float32x2 }, // uv_offset
                wgpu::VertexAttribute { shader_location: 5, offset: 24, format: wgpu::VertexFormat::Float32x2 }, // uv_scale
                wgpu::VertexAttribute { shader_location: 6, offset: 32, format: wgpu::VertexFormat::Float32 }, // rotation
                wgpu::VertexAttribute { shader_location: 7, offset: 36, format: wgpu::VertexFormat::Float32x4 }, // color
            ],
        }
    }
}