use crate::Component;
use crate::runtime::assets::AssetHandle;
use super::PipelineKey;

#[derive(Component!)]
pub struct BatchHandle {
    pub batch_index: usize,
    pub index: usize,
    pub asset_handle: AssetHandle,   
    pub pipeline_key: PipelineKey,
}

impl BatchHandle {
    pub fn new(asset_handle: AssetHandle, pipeline_key: PipelineKey) -> Self {
        Self { batch_index: 0, index: usize::MAX, asset_handle, pipeline_key}
    }
}