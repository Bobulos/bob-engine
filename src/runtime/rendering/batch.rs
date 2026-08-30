use super::PipelineKey;
use wgpu::{BindGroup, Buffer};

pub struct Batch {
    pub pipeline_key: PipelineKey,
    pub instances: Vec<u8>,
    pub instance_stride: usize,
    pub instance_buffer: Buffer,
    pub instance_capacity: usize, // track buffer size to know when to reallocate
    pub num_instances: u32,
    pub bind_group: BindGroup,
    pub _texture: crate::runtime::rendering::texture::Texture, // keeps GPU texture alive
}