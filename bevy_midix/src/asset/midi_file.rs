#![doc = r#"
Asset types

TODO
"#]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};

use midix::{events::LiveEvent, file::MidiFile as Mf, reader::ReaderError};

use crate::synth::{MidiCommandSource, SinkCommand, SinkCommands};

/// Sound font asset. Wraps a midix MidiFile
#[derive(Asset, TypePath)]
pub struct MidiFile {
    inner: Mf<'static>,
}

impl MidiFile {
    /// Create a new midifile with the given inner midix MidiFile
    pub fn new(file: Mf<'static>) -> Self {
        Self { inner: file }
    }

    /// Get a reference to the inner midifile
    pub fn inner(&self) -> &Mf<'static> {
        &self.inner
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

impl MidiCommandSource for MidiFile {
    fn to_commands(&self) -> crate::prelude::SinkCommands {
        let midi = &self.inner;

        let mut commands = Vec::new();
        let tracks = midi.tracks();

        let ticks_per_qn = midi.header().timing().ticks_per_quarter_note().unwrap();

        let bpm = 120.;
        // assume 4 quarter notes per measure

        //so one beat is a quarter note
        // so quarter_notes_per_minute
        // quarter_notes_per_second
        let micros_per_quarter_note = 0.0002 / bpm;

        for track in tracks {
            for event in track.events() {
                match event.event() {
                    LiveEvent::ChannelVoice(cv) => {
                        let tick = event.accumulated_ticks();
                        let quarter_notes = tick as f64 / ticks_per_qn as f64;
                        let micros = quarter_notes / micros_per_quarter_note;
                        commands.push(SinkCommand::new(micros as u64, *cv));
                    }
                    _ => {
                        //idk
                    }
                }
            }
        }
        SinkCommands::new(commands)
    }
}
