
use crate::runtime::assets::asset_handle::AssetType;
use crate::runtime::assets::{Asset, AssetEmbedded, AssetHandle};
use std::{collections::HashMap, hash::Hasher};

#[derive(Debug, Clone)]
pub struct AssetStore {

    /// Should get rid of these but for right 
    /// now it works for local storage
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
        self.assets.get(idx.idx)
    }
    pub fn get_asset_idx(&self, hash: u64) -> Option<AssetHandle> {
        self.asset_handles.get(&hash).copied()
    }
    pub fn get_asset_handle_by_path(&self, path: &str) -> Option<AssetHandle> {
        let hash = self.path_to_hash.get(path)?;
        self.get_asset_idx(*hash)
    }
    pub fn get_asset_hash(&self, path: &str) -> u64 {
        generate_hash(path)
    }

    pub fn get_asset_by_hash(&self, hash: u64) -> Option<&Asset> {
        let idx = self.asset_handles.get(&hash)?;
        self.assets.get(idx.idx)
    }

    pub fn get_asset_by_path(&self, path: &str) -> Option<&Asset> {
        let hash = self.path_to_hash.get(path)?;
        self.get_asset_by_hash(*hash)
    }
    /// should include all assets before calling this
    pub fn init(&mut self) {
        self.load_include_cfg();
        self.generate_assets();
    }

    /// Modern can be used to manualy load
    pub fn include_asset(&mut self, path: &'static str, data_raw: &[u8]) {
        let hash = generate_hash(path);

        let data = data_raw.to_vec();
        
        let path_as_string = String::from(path);
        let asset = Asset::new(hash, path_as_string.clone(), Some(data));

        let index = self.assets.len();

        self.assets.push(asset);
        
        let handle = AssetHandle::new(index, get_asset_type_from_path(path_as_string.clone()));
        self.asset_handles.insert(hash, handle);
        println!("include asset: IDX:{}, HASH:{}, TYPE:{:?}, PATH:{}", handle.idx, hash, handle.file_type, path);
        
        self.path_to_hash.insert(path_as_string.clone(), hash);
    }
    //
    // Needs to go
    // 
    pub fn load_include_cfg(&mut self) {
        let raw_json_data = AssetEmbedded::get(ASSET_STORE_PATH)
            .expect("Couldn't load asset_store.json")
            .data
            .into_owned();

        let asset_paths: Vec<String> =
            serde_json::from_slice(&raw_json_data).expect("Failed to parse asset_store.json");
        self.included_paths = asset_paths;
        //println!("Assets included:");
    }

    //
    // KILL THIS REPLACED BY INCLUDE ASSET
    // 
    
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
            
            let handle = AssetHandle::new(index, get_asset_type_from_path(path.clone()));
            self.asset_handles.insert(hash, handle);
            println!("include asset: IDX:{}, HASH:{}, TYPE:{:?}, PATH:{}", handle.idx, hash, handle.file_type, path);
            
            self.path_to_hash.insert(path.clone(), hash);
        }
    }
}

pub fn generate_hash(item: &str) -> u64 {
    let mut hasher = seahash::SeaHasher::new();
    hasher.write(item.as_bytes());
    hasher.finish()
}

pub fn get_asset_type_from_path(path: String) -> Option<AssetType> {
    let extension = get_extension_string(&path);
    let t = match extension {
        Some("png") => Some(AssetType::Png),
        Some("jpeg") => Some(AssetType::Jpeg),
        Some("jpg") => Some(AssetType::Jpeg),
        Some("bscene") => Some(AssetType::BScene),
        Some("json") => Some(AssetType::Json),
        None => None,
        _ => None,
    };
    t
}

pub fn get_extension_string(filename: &String) -> Option<&str> {
    filename
        .rfind('.')
        .map(|idx| &filename[idx + 1..])
}