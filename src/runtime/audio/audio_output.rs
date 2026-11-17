use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::io::Cursor;
use std::io::BufReader;
use std::sync::OnceLock;
use std::sync::Arc;
use super::super::assets::AssetStore;


pub struct AudioOutput {
    handle: MixerDeviceSink,
    asset_store: Option<Arc<OnceLock<AssetStore>>>,
}

impl AudioOutput {
    pub fn new() -> Self {
        Self { 
            handle: rodio::DeviceSinkBuilder::open_default_sink()
                .expect("open default audio stream"),
        }
    }
    pub fn play_sound(&self) {
        let cursor = Cursor::new([2; 1024]);
        let source = Decoder::new(cursor).expect("Could not decode audio file");

        self.handle.mixer().add(source);
    }
}