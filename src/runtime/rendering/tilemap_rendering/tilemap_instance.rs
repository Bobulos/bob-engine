use wgpu::VertexBufferLayout;

use crate::runtime::rendering::VertexLayout;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TilemapInstance {
    
}
impl VertexLayout for TilemapInstance {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TilemapInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // i_pos: @location(2)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // i_size: @location(3)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // i_uv_offset: @location(4)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // i_uv_scale: @location(5)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // rotation: @location(6)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}