use crate::runtime::assets::asset_store::generate_hash;
use crate::runtime::assets::AssetStore;


// #[macro_export]
// macro_rules! include_assets {
//     ($a:expr, $list:expr, $length:expr) => {
//         let asset_store: &mut AssetStore = $a;
//         let length: usize = $length;
//         let list: [&'static str; 2] = $list;
//         let i: usize = 0;
//         while i < length {
//             include_asset!(asset_store, list[i]);
//         }
//     };
// }

#[macro_export]
/// Takes an asset store and a path
macro_rules! include_asset {
    ($a:expr, $p:expr) => {
        {
            let _asset_store: &mut AssetStore = $a;
            //let path:  = $p;
            let _path: &str = $p;
            $a.include_asset($p, include_bytes!($p));
        }
    };
}