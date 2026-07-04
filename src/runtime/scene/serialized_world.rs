use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SerializedWorld {
    /// Initialized entities 0..entity_count
    pub entity_count: u64,
    /// Holds arrays of component def's by stable component id
    pub component_defs: HashMap<u64, HashMap<u64, serde_json::Value>>,

}
