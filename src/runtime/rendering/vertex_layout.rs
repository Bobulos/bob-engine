pub trait VertexLayout {
    fn layout() -> wgpu::VertexBufferLayout<'static>;
}