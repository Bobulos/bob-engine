use crate::runtime::assets::{Asset, AssetEmbedded, AssetHandle};
use std::{collections::HashMap, hash::Hasher};

#[derive(Debug)]
pub struct AssetStore {
    pub included_paths: Vec<String>,

    // hash = asset idx
    asset_handles: HashMap<u64, AssetHandle>,

    assets: Vec<Asset>,

    // path = hash
    path_to_hash: HashMap<String, u64>,
}

const ASSET_STORE_PATH: &str = "cfg/asset_store.json";

impl AssetStore {
    pub fn new() -> Self {
        Self {
            included_paths: Vec::new(),
            asset_handles: HashMap::new(),
            assets: Vec::new(),
            path_to_hash: HashMap::new(),
        }
    }
    pub fn get_asset_by_handle(&self, idx: AssetHandle) -> Option<&Asset> {
        self.assets.get(idx.0)
    }
    pub fn get_asset_idx(&self, hash: u64) -> Option<AssetHandle> {
        self.asset_handles.get(&hash).copied()
    }
    pub fn get_asset_idx_by_path(&self, path: &str) -> Option<AssetHandle> {
        let hash = self.path_to_hash.get(path)?;
        self.get_asset_idx(*hash)
    }
    pub fn get_asset_hash(&self, path: &str) -> u64 {
        generate_hash(path)
    }

    pub fn get_asset_by_hash(&self, hash: u64) -> Option<&Asset> {
        let idx = self.asset_handles.get(&hash)?;
        self.assets.get(idx.0)
    }

    pub fn get_asset_by_path(&self, path: &str) -> Option<&Asset> {
        let hash = self.path_to_hash.get(path)?;
        self.get_asset_by_hash(*hash)
    }

    pub fn init(&mut self) {
        self.load_include_cfg();
        self.generate_assets();
    }

    pub fn load_include_cfg(&mut self) {
        let raw_json_data = AssetEmbedded::get(ASSET_STORE_PATH)
            .expect("Couldn't load asset_store.json")
            .data
            .into_owned();

        let asset_paths: Vec<String> =
            serde_json::from_slice(&raw_json_data).expect("Failed to parse asset_store.json");

        println!("Assets included:");

        for (idx, path) in asset_paths.iter().enumerate() {
            println!("{}, Hash: {}, Idx: {}", path, generate_hash(&path), idx);
            self.included_paths.push(path.clone());
        }
    }

    fn generate_assets(&mut self) {
        for path in &self.included_paths {
            let hash = generate_hash(path);

            let data = AssetEmbedded::get(path)
                .expect("Failed to load asset")
                .data
                .into_owned();

            let asset = Asset::new(hash, path.clone(), Some(data));

            let index = self.assets.len();

            self.assets.push(asset);

            self.asset_handles.insert(hash, AssetHandle(index));
            self.path_to_hash.insert(path.clone(), hash);
        }
    }
}

fn generate_hash(item: &str) -> u64 {
    let mut hasher = seahash::SeaHasher::new();
    hasher.write(item.as_bytes());
    hasher.finish()
}
