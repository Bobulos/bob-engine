use crate::Component;
use crate::runtime::assets::AssetHandle;

#[derive(Component!)]
pub struct BatchHandle {
    pub batch_index: usize,
    pub index: usize,
    pub asset_handle: AssetHandle,   
}

impl BatchHandle {
    pub fn new(asset_handle: AssetHandle) -> Self {
        Self { batch_index: 0, index: usize::MAX, asset_handle,}
    }
}