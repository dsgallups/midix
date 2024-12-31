#![doc = r#"
Asset types

TODO
"#]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::convert::Infallible;
use thiserror::Error;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};

/// Sound font asset
#[derive(Asset, TypePath)]
pub struct MidiFile {
    file: Vec<u8>,
}

impl MidiFile {
    /// Create a new
    fn new(file: Vec<u8>) -> Self {
        Self { file }
    }
}

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
enum MidiFileLoadError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

/// Loader for sound fonts
#[derive(Default)]
pub struct MidiFileLoader;

impl AssetLoader for MidiFileLoader {
    type Asset = MidiFile;
    type Settings = ();
    type Error = Infallible;
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

        let res = MidiFile::new(bytes);

        Ok(res)
    }

    fn extensions(&self) -> &[&str] {
        &["mid"]
    }
}
