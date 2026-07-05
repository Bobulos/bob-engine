use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum AssetType {
    Png,
    Jpeg,
    Json,
    BScene,
}
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct AssetHandle {
    pub idx: usize,
    pub file_type: Option<AssetType>,
}
impl AssetHandle {
    pub fn new(idx: usize, file_type: Option<AssetType>) -> Self{
        Self { idx, file_type }
    }
}
