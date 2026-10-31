use crate::runtime::rendering::InstanceLayout;
use wgpu::VertexFormat::*;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GuiShapeInstance {
    pub position: [f32; 2],     // location 2
    pub size: [f32; 2],         // location 3
    pub uv_offset: [f32; 2],    // location 4
    pub uv_scale: [f32; 2],     // location 5
    pub corner_radius: f32,     // location 6
    pub color: [f32; 4],        // location 7
    pub border_width: f32,      // location 8
    pub border_color: [f32; 4], // location 9
    pub _pad: [u32; 3],         // realign to 16 bytes
}

impl InstanceLayout for GuiShapeInstance {
    fn bind_group(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, cam_buffer: &wgpu::Buffer, texture: &crate::runtime::rendering::texture::Texture) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Batch Bind Group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: cam_buffer.as_entire_binding(),
                },
            ],
        }) 
    }
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    // 0 texture
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
                    // 1 sampler
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    // 2 camera uniform
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
        
        bind_group_layout
    }
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<GuiShapeInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: 0,
                    format: Float32x2,
                }, // position
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: 8,
                    format: Float32x2,
                }, // size
                wgpu::VertexAttribute {
                    shader_location: 4,
                    offset: 16,
                    format: Float32x2,
                }, // uv_offset
                wgpu::VertexAttribute {
                    shader_location: 5,
                    offset: 24,
                    format: Float32x2,
                }, // uv_scale
                wgpu::VertexAttribute {
                    shader_location: 6,
                    offset: 32,
                    format: Float32,
                }, // corner_radius
                wgpu::VertexAttribute {
                    shader_location: 7,
                    offset: 36,
                    format: Float32x4,
                }, // color
                wgpu::VertexAttribute {
                    shader_location: 8,
                    offset: 52,
                    format: Float32,
                }, // border_width
                wgpu::VertexAttribute {
                    shader_location: 9,
                    offset: 56,
                    format: Float32x4,
                }, // border_color
            ],
        }
    }
}
impl Default for GuiShapeInstance {
    fn default() -> Self {
        Self {
            position: [f32::MIN, f32::MIN],
            size: [100.0, 100.0],
            uv_offset: [0.0, 0.0],
            uv_scale: [1.0, 1.0],
            corner_radius: 1.0,
            color: [1.0, 0.0, 0.0, 1.0],
            border_width: 0.3,
            border_color: [1.0, 1.0, 1.0, 1.0],
            _pad: [0; 3],
        }
    }
}
