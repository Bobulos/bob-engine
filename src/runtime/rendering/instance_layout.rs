use wgpu;

use crate::runtime::rendering::texture::{self, Texture};

pub trait InstanceLayout {
    fn bind_group(device: &wgpu::Device, cam_buffer: &wgpu::Buffer, texture: &Texture) -> wgpu::BindGroup;
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static>;
}
