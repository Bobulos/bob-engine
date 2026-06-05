use crate::runtime::assets::AssetHandle;
use crate::runtime::ecs::Entity;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::rendering::sprite_rendering::atlas_handle::AtlasHandle;
use crate::runtime::rendering::sprite_rendering::components::Sprite;
use crate::runtime::rendering::{Instance, Renderer};
use crate::runtime::{self, rendering};
use std::sync::{Arc, RwLock};

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
        println!("Allocating new atlas for: {}", asset_handle.0);
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
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

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
            let matching_idx = self
                .asset_handles
                .iter()
                .enumerate()
                .find(|(idx, handle)| {
                    handle.0 == sprite_asset_handle.0
                        && self.atlas_idxs[*idx] < MAX_SPRITES_PER_BATCH
                })
                .map(|(idx, _)| idx);

            if let Some(mut matching_idx) = matching_idx {
                if self.atlas_idxs[matching_idx] >= MAX_SPRITES_PER_BATCH {
                    matching_idx = self.allocate_atlas(&sprite_asset_handle);
                }
                // Handle batch overflow
                // Assign more sprites to a different batch
                world.get_component_mut(entity, |sprite: &mut Sprite| {
                    sprite.batch_index = self.atlas_handles[matching_idx].0;
                    sprite.index = self.atlas_idxs[matching_idx];
                });
                self.atlas_idxs[matching_idx] += 1;
            } else {
                self.allocate_atlas(&sprite_asset_handle);
            }
        }
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}
