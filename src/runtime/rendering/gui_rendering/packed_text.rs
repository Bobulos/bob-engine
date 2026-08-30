use serde_with::{serde_as, Bytes};
use serde;


#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct PackedText<const N: usize> {
    #[serde_as(as = "Bytes")] // Much faster specialized byte-array handling
    pub packed: [u8; N],
}

impl<const N: usize> PackedText<N> {
    pub fn new(packed: [u8; N]) -> Self {
        Self {
            packed
        }
    }
}
impl<const N: usize> Default for PackedText<N> {
    fn default() -> Self {
        Self { 
            packed: [0; N] 
        }
    }
}