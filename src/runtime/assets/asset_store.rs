use crate::runtime::assets::{Asset, AssetEmbedded};
use std::{collections::HashMap, hash::Hasher};

pub struct AssetStore {
    /// Baked from the asset_store.json config file.
    pub included_paths: Vec<String>,
    assets: HashMap<u64, Asset>,
}

// Raw: assets/cfg/asset_store.json
const ASSET_STORE_PATH: &str = "cfg/asset_store.json";
impl AssetStore {
    pub fn new() -> Self {
        Self {
            included_paths: Vec::new(),
            assets: HashMap::new(),
        }
    }
    /// Loads assets into memory.
    pub fn init(&mut self) {
        self.load_include_cfg();
        self.generate_assets();
    }
    pub fn load_include_cfg(&mut self) {
        let raw_json_data = AssetEmbedded::get(ASSET_STORE_PATH);
        let unwrapped_json_data = raw_json_data
            .expect("Couldn't load the asset_store.json")
            .data
            .into_owned();
        let mut decoded_json: Option<Vec<String>> = None;
        match str::from_utf8(&unwrapped_json_data) {
            Ok(string_slice) => {
                decoded_json = Some(serde_json::from_str(string_slice).unwrap());
            }
            Err(e) => println!("Error couldnt decode json: {}", e),
        }

        if let Some(asset_paths) = decoded_json {
            println!("Assets included:");
            for path in &asset_paths {
                println!("{}", path);
                self.included_paths.push(path.clone());
            }
        }
    }
    fn generate_assets(&mut self) {
        for path in &self.included_paths {
            let hash = generate_hash(path);
            let data = AssetEmbedded::get(path)
                .expect("Failed to unrwap asset data.")
                .data
                .into_owned();
            self.assets
                .insert(hash, Asset::new(hash, path.clone(), Some(data)));
        }
    }
}
fn generate_hash(item: &str) -> u64 {
    let mut hasher = seahash::SeaHasher::new();
    hasher.write(item.as_bytes());
    hasher.finish()
}
