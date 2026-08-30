use crate::runtime::rendering::VertexLayout;
use wgpu::VertexFormat::*;
use wgpu::VertexAttribute;
use std::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GuiCharInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
    // mabeye add rot later
}
impl Default for GuiCharInstance {
    fn default() -> Self {
        Self { 
            position: [f32::MAX, f32::MAX], 
            size: [1.0, 1.0], 
            uv_offset: [0.0, 0.0], 
            uv_scale: [1.0, 1.0] 
        }   
    }
}
impl VertexLayout for GuiCharInstance {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout { 
            array_stride: size_of::<GuiCharInstance>() as u64, 
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
            ],
        }
    }
}
