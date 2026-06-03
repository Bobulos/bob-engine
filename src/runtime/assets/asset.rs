#[derive(Debug, Clone)]
pub struct Asset {
    pub hash: u64,
    pub path: String,
    pub data: Option<Vec<u8>>,
}
impl Asset {
    pub fn new(hash: u64, path: String, data: Option<Vec<u8>>) -> Self {
        Self { hash, path, data }
    }
}
