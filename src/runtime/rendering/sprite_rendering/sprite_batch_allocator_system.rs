use crate::runtime::assets::{AssetHandle, AssetStore, asset_handle};
use crate::runtime::ecs::Entity;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::rendering::sprite_rendering::atlas_handle::AtlasHandle;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use crate::runtime::rendering::{Instance, Renderer};
use crate::runtime::{self, rendering};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

pub const MAX_ATLASES_PER_SPRITE: usize = 16;
pub const MAX_SPRITES_PER_BATCH: usize = 0xfff;
const UNASSIGNED: usize = usize::MAX;

pub struct SpriteBatchAllocatorSystem {
    pub renderer: Arc<RwLock<Renderer>>,

    /// Maps inner value maps to renderer batches.
    atlas_handles: Vec<AtlasHandle>,
    atlas_idxs: Vec<usize>,
    /// Maps atlas handles to sprite asset handles.
    asset_handles: Vec<AssetHandle>,
}

impl SpriteBatchAllocatorSystem {
    pub fn new(renderer: Arc<RwLock<Renderer>>) -> Self {
        Self {
            renderer,
            atlas_idxs: Vec::new(),
            atlas_handles: Vec::new(),
            asset_handles: Vec::new(),
        }
    }

    // Renderer interface --------------------------------------------
    /// Allocates a new atlas handle and returns its index.
    pub fn allocate_atlas(&mut self, asset_handle: &AssetHandle) -> usize {
        self.atlas_handles
            .push(AtlasHandle(self.allocate_batch(asset_handle.clone())));
        self.asset_handles.push(asset_handle.clone());
        self.atlas_idxs.push(0);
        self.atlas_handles.len() - 1
    }
    fn allocate_batch(&self, asset_handle: AssetHandle) -> usize {
        let new_batch_id = {
            let mut renderer_lock = self.renderer.write().unwrap();
            renderer_lock.create_batch(
                asset_handle,
                vec![Instance::default(); runtime::engine::SPRITE_BATCH_SIZE],
                rendering::renderer::PipelineKey::Default,
            )
        };
        return new_batch_id;
    }
}

impl SystemBase for SpriteBatchAllocatorSystem {
    fn on_start(&mut self, world: &Arc<DynamicWorld>) {}

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut pending: Vec<(Entity, AssetHandle)> = Vec::new();

        world.for_each_mut::<Sprite>(|_entity, sprite| {
            if sprite.index == UNASSIGNED {
                pending.push((
                    _entity,
                    // Fix this jhon
                    sprite.asset_handle,
                ));
            }
        });
        for (entity, sprite_asset_handle) in pending {
            let mut matching_idx = self
                .asset_handles
                .iter()
                .position(|h| h.0 == sprite_asset_handle.0);

            if let Some(matching_idx) = matching_idx {
                if self.atlas_idxs[matching_idx] > MAX_SPRITES_PER_BATCH {
                    matching_idx = self.allocate_atlas(&sprite_asset_handle);
                }
                // Handle batch overflow
                // Assign more sprites to a different batch
                world.get_component_mut(entity, |sprite: &mut Sprite| {
                    sprite.batch_index = self.atlas_handles[matching_idx].0;
                    sprite.index = self.atlas_idxs[matching_idx];
                });
            } else {
                self.allocate_atlas(&sprite_asset_handle);
            }
            for (idx, asset_handle) in self.asset_handles.iter().enumerate() {
                // Allocate the sprite to the atlas
                if asset_handle.0 == sprite_asset_handle.0 {
                    let mut batch_assignment = 0;
                    if self.atlas_idxs[idx] > MAX_SPRITES_PER_BATCH {
                        batch_assignment = self.allocate_atlas(&asset_handle);
                    } else {
                        batch_assignment = idx;
                    }
                    // Handle batch overflow
                    // Assign more sprites to a different batch
                    world.get_component_mut(entity, |sprite: &mut Sprite| {
                        sprite.batch_index = self.atlas_handles[batch_assignment].0;
                        sprite.index = self.atlas_idxs[batch_assignment];
                    });
                }
                self.atlas_idxs[idx] += 1;
            }
        }
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}

// fn allocate_slot(&mut self, atlas_handle: AssetHandle) -> (usize, usize) {
//     let batches = &self.atlas_batches[atlas_id];
//     let slots = &self.atlas_next_slot[atlas_id];

//     for batch_idx in 0..batches.len() {
//         if slots[batch_idx] < runtime::engine::SPRITE_BATCH_SIZE {
//             let slot = self.atlas_next_slot[atlas_id][batch_idx];
//             self.atlas_next_slot[atlas_id][batch_idx] += 1;
//             return (batches[batch_idx], slot);
//         }
//     }

//     // overflow -> create new batch from AssetStore
//     let path = &self.atlas_paths[atlas_id];
//     let store = self.asset_store.get().unwrap();

//     let asset = store
//         .get_asset_by_path(path)
//         .expect("Atlas missing in AssetStore");

//     let bytes = asset.data.as_ref().expect("Atlas has no data");

//     println!("Atlas {} full, creating overflow batch", atlas_id);

//     let new_batch_id = {
//         let mut renderer_lock = self.renderer.write().unwrap();
//         renderer_lock.create_batch(
//             bytes,
//             vec![Instance::default(); runtime::engine::SPRITE_BATCH_SIZE],
//             rendering::renderer::PipelineKey::Default,
//         )
//     };

//     self.atlas_batches[atlas_id].push(new_batch_id);
//     self.atlas_next_slot[atlas_id].push(1);

//     (new_batch_id, 0)
// }
