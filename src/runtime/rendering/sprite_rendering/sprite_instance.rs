use wgpu::{BindGroup, wgc::id::markers::BindGroupLayout};

use crate::runtime::rendering::InstanceLayout;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstance {
    pub position: [f32; 2],  // world position
    pub size: [f32; 2],      // width, height scale
    pub uv_offset: [f32; 2], // for sprite sheets, else [0.0, 0.0]
    pub uv_scale: [f32; 2],  // for sprite sheets, else [1.0, 1.0]
    pub rotation: f32,
    //pub _padding: [f32; 3],
}
impl SpriteInstance {
    pub fn default() -> Self {
        Self {
            position: [f32::MAX, f32::MAX],
            size: [1.0, 1.0],
            uv_scale: [1.0, 1.0],
            uv_offset: [0.0, 0.0],
            rotation: 0.0,
        }
    }
}
impl InstanceLayout for SpriteInstance {
    fn bind_group_layout(device: &wgpu::Device, cam_entry: &wgpu::BindGroupLayoutEntry) -> BindGroup {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // 0 — texture
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // 1 — sampler
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // 2 — camera uniform
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        
    }
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteInstance>() as wgpu::BufferAddress,
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