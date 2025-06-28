#![doc = r#"
Asset types

TODO
"#]

use alloc::sync::Arc;
use bevy_platform::prelude::*;
use thiserror::Error;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use rustysynth::SoundFont as Sf;

/// Sound font asset
#[derive(Asset, TypePath)]
pub struct SoundFontAsset {
    /// The inner parsed soundfont
    pub file: Arc<Sf>,
}

impl SoundFontAsset {
    /// Create a new
    fn new(file: &mut &[u8]) -> Self {
        let sf = Sf::new(file).unwrap();

        Self { file: Arc::new(sf) }
    }
}
/// Possible errors that can be produced by [`CustomAssetLoader`]
#[derive(Debug, Error)]
pub enum SoundFontLoadError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

/// Loader for sound fonts
#[derive(Default)]
pub struct SoundFontLoader;

impl AssetLoader for SoundFontLoader {
    type Asset = SoundFontAsset;
    type Settings = ();
    type Error = SoundFontLoadError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        info!(
            "Loading bytes...this might take a while. If taking too long, run with --release or with opt-level = 3!"
        );
        reader.read_to_end(&mut bytes).await?;

        info!("Loaded!");
        let res = SoundFontAsset::new(&mut bytes.as_slice());

        Ok(res)
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
