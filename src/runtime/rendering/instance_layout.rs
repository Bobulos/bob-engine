use wgpu;

pub trait InstanceLayout {
    fn bind_group_layout(device: &wgpu::Device, cam_entry: &wgpu::BindGroupLayoutEntry) -> wgpu::BindGroupLayout;
    fn vertex_layout() -> wgpu::VertexBufferLayout<'static>;
}
