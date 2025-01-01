#![doc = r#"
Asset types

TODO
"#]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};

use midix::{file::MidiFile as Mf, reader::ReaderError};

/// Sound font asset
#[derive(Asset, TypePath)]
pub struct MidiFile {
    inner: Mf<'static>,
}

impl MidiFile {
    /// Create a new
    pub fn new(file: Mf<'static>) -> Self {
        Self { inner: file }
    }
}

/// Loader for sound fonts
#[derive(Default)]
pub struct MidiFileLoader;

impl AssetLoader for MidiFileLoader {
    type Asset = MidiFile;
    type Settings = ();
    type Error = ReaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.unwrap();

        let inner = Mf::parse(bytes)?;

        let res = MidiFile::new(inner);

        Ok(res)
    }

    fn extensions(&self) -> &[&str] {
        &["mid"]
    }
}
