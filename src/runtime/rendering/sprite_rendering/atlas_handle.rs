use crate::runtime::rendering::PipelineKey;

#[derive(Debug, Clone, PartialEq)]
/// Stores the renderer's batch index for a given atlas.
pub struct AtlasHandle {
    pub idx: usize,
    pub pipeline_key: PipelineKey,
}
impl AtlasHandle {
    pub fn new(idx: usize, pipeline_key: PipelineKey) -> Self{
        Self { idx, pipeline_key }
    }
}