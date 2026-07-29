use crate::runtime::assets::asset_store::generate_hash;
use crate::runtime::assets::AssetStore;
#[macro_export]
/// Takes an asset store and a path
macro_rules! include_asset {
    ($a:expr, $p:expr) => {
        {
            let asset_store: &mut AssetStore = $a;
            //let path:  = $p;
            let path: &str = $p;
            $a.include_asset($p, include_bytes!($p));
        }
    };
}