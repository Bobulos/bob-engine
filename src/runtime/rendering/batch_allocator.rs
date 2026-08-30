use crate::runtime::assets::AssetHandle;
use crate::runtime::ecs::Entity;
use crate::runtime::ecs::{DynamicWorld, SystemBase};
use crate::runtime::rendering::gui_rendering::GuiCharInstance;
use crate::runtime::rendering::gui_rendering::gui_shape_instance::GuiShapeInstance;
use super::PipelineKey;
use crate::runtime::rendering::sprite_rendering::atlas_handle::AtlasHandle;
use crate::runtime::rendering::{BatchHandle, Renderer};
use crate::runtime::rendering::sprite_rendering::SpriteInstance;
use crate::runtime::{self};
use std::sync::{Arc, RwLock};

pub const MAX_PER_BATCH: usize = 0xfff;
const UNASSIGNED: usize = usize::MAX;

pub struct BatchAllocator {
    pub renderer: Arc<RwLock<Renderer>>,

    /// Maps inner value maps to renderer batches.
    atlas_handles: Vec<AtlasHandle>,
    atlas_idxs: Vec<usize>,
    /// Maps atlas handles to sprite asset handles.
    asset_handles: Vec<AssetHandle>,
}

impl BatchAllocator {
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
    pub fn allocate_atlas(&mut self, asset_handle: &AssetHandle, pipeline_key: PipelineKey) -> usize {
        let batch = self.allocate_batch(asset_handle.clone(), pipeline_key);
        println!("Allocating new atlas for: {},{:?},{}", asset_handle.idx, pipeline_key, batch);
        self.atlas_handles
            .push(AtlasHandle::new(batch, pipeline_key));
        self.asset_handles.push(asset_handle.clone());
        self.atlas_idxs.push(0);
        self.atlas_handles.len() - 1
    }
    fn allocate_batch(&self, asset_handle: AssetHandle, pipeline_key: PipelineKey) -> usize {
        let mut renderer_lock = self.renderer.write().unwrap();
        match pipeline_key {
            PipelineKey::GuiShape => {
                let instances = vec![GuiShapeInstance::default(); runtime::engine::RENDER_BATCH_SIZE];
                renderer_lock.create_batch(asset_handle, &instances, pipeline_key)
            }
            PipelineKey::GuiText => {
                let instances = vec![GuiCharInstance::default(); runtime::engine::RENDER_BATCH_SIZE];
                renderer_lock.create_batch(asset_handle, &instances, pipeline_key)
            }
            _ => {
                let instances = vec![SpriteInstance::default(); runtime::engine::RENDER_BATCH_SIZE];
                renderer_lock.create_batch(asset_handle, &instances, pipeline_key)
            }
        }
    }
}

impl SystemBase for BatchAllocator {
    fn on_start(&mut self, _world: &Arc<DynamicWorld>) {}

    fn on_update(&mut self, world: &Arc<DynamicWorld>) {
        let mut pending: Vec<(Entity, AssetHandle, PipelineKey)> = Vec::new();
        world.for_each_mut::<BatchHandle>(|_entity, handle| {
            if handle.index == UNASSIGNED {
                pending.push((
                    _entity,
                    // Fix this jhon
                    handle.asset_handle,
                    handle.pipeline_key,
                ));
            }
        });

        for (entity, batch_handle_asset_handle, pipeline_key) in pending {
            let matching_idx = self
                .asset_handles
                .iter()
                .enumerate()
                .find(|(idx, handle)| {
                    handle.idx == batch_handle_asset_handle.idx
                        && self.atlas_handles[*idx].pipeline_key == pipeline_key
                        && self.atlas_idxs[*idx] < MAX_PER_BATCH
                })
                .map(|(idx, _)| idx);

            if let Some(mut matching_idx) = matching_idx {
                if self.atlas_idxs[matching_idx] >= MAX_PER_BATCH {
                    matching_idx = self.allocate_atlas(&batch_handle_asset_handle, pipeline_key);
                }
                // Handle batch overflow
                // Assign more sprites to a different batch
                world.get_component_mut(entity, |handle: &mut BatchHandle| {
                    handle.batch_index = self.atlas_handles[matching_idx].idx;
                    handle.index = self.atlas_idxs[matching_idx];
                });
                self.atlas_idxs[matching_idx] += 1;
            } else {
                self.allocate_atlas(&batch_handle_asset_handle, pipeline_key);
            }
        }
    }
    fn on_destroy(&mut self, _world: &Arc<DynamicWorld>) {}
}