#![doc = r#"
Asset types

TODO
"#]

use std::{io::Read, sync::Arc};
use thiserror::Error;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use midix_synth::{prelude::SoundFontError, soundfont::SoundFont as Sf};

/// Sound font asset
#[derive(Asset, TypePath)]
pub struct SoundFont {
    file: Arc<Sf>,
}

impl SoundFont {
    /// Create a new
    fn new<R: Read + ?Sized>(file: &mut R) -> Result<Self, SoundFontError> {
        let sf = Arc::new(Sf::new(file)?);

        Ok(Self { file: sf })
    }
    /// Provides the interior font
    pub fn font(&self) -> &Arc<Sf> {
        &self.file
    }
}

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
enum SoundFontLoadError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

/// Loader for sound fonts
#[derive(Default)]
pub struct SoundFontLoader;

impl AssetLoader for SoundFontLoader {
    type Asset = SoundFont;
    type Settings = ();
    type Error = SoundFontError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        println!("here");
        reader.read_to_end(&mut bytes);
        println!("read to end");

        let res = SoundFont::new(&mut bytes.as_slice())?;

        Ok(res)
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
