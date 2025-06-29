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

use crate::{
    events::LiveEvent,
    file::ParsedMidiFile as Mf,
    prelude::{FormatType, Timed, Timing},
    reader::ReaderError,
};

use crate::bevy::song::MidiSong;

/// Sound font asset. Wraps a midix MidiFile
///
/// TODO(before v4: do not wrap midix MidiFile)
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

    /// uses owned self to make a song sendable to the synth
    pub fn into_song(self) -> MidiSong {
        self.inner.into()
    }
    /// uses reference to make a song
    pub fn to_song(&self) -> MidiSong {
        (&self.inner).into()
    }
}

impl<'a> From<Mf<'a>> for MidiSong {
    fn from(midi: Mf<'a>) -> Self {
        let mut commands = Vec::new();
        let tracks = midi.tracks();

        // is Some if the tempo is set for the whole file.
        // None if the format is sequentially independent
        let file_tempo = match midi.format_type() {
            FormatType::SequentiallyIndependent => None,
            FormatType::Simultaneous | FormatType::SingleMultiChannel => {
                let first_track = tracks.first().unwrap();
                Some(first_track.info().tempo)
            }
        };

        for track in tracks {
            let track_tempo = file_tempo.unwrap_or(track.info().tempo);
            let micros_per_quarter_note = track_tempo.micros_per_quarter_note();

            let (micros_per_tick, offset_in_micros) = match midi.header().timing() {
                Timing::Smpte(v) => {
                    //µs_per_tick = 1 000 000 / (fps × ticks_per_frame)
                    //FPS is −24/−25/−29/−30 in the high byte of division;
                    // ticks per frame is the low byte.

                    let frames_per_second = v.fps().as_division() as u32;
                    let ticks_per_frame = v.ticks_per_frame() as u32;
                    let ticks_per_second = frames_per_second * ticks_per_frame;
                    let micros_per_tick = 1_000_000. / ticks_per_second as f64;

                    //NOTE: if the file header uses smpte, that overrides any track smpte offset.
                    let offset_in_micros = track
                        .info()
                        .smpte_offset
                        .as_ref()
                        .map(|offset| {
                            if offset.fps != v.fps() {
                                warn!(
                                    "Header's fps({}) does not align with track's fps({}). \
                                    The file's fps will override the track's!",
                                    v.fps().as_f64(),
                                    offset.fps.as_f64()
                                );
                            }
                            offset.as_micros_with_override(v.fps())
                        })
                        .unwrap_or(0.);

                    (micros_per_tick, offset_in_micros)
                }
                Timing::TicksPerQuarterNote(tpqn) => {
                    // µs_per_tick = tempo_meta / TPQN
                    // micro_seconds/quarternote * quarternote_per_tick (1/ticks per qn)
                    let micros_per_tick =
                        micros_per_quarter_note as f64 / tpqn.ticks_per_quarter_note() as f64;

                    let offset_in_micros = track
                        .info()
                        .smpte_offset
                        .as_ref()
                        .map(|offset| offset.as_micros())
                        .unwrap_or(0.);

                    (micros_per_tick, offset_in_micros)
                }
            };

            for event in track.events() {
                match event.event() {
                    LiveEvent::ChannelVoice(cv) => {
                        let tick = event.accumulated_ticks();
                        let micros = micros_per_tick * tick as f64;

                        commands.push(Timed::new(micros as u64, *cv));
                    }
                    _ => {
                        //idk
                    }
                }
            }
        }
        MidiSong::new(commands)
    }
}

impl<'a> From<&Mf<'a>> for MidiSong {
    fn from(midi: &Mf<'a>) -> Self {
        let mut commands = Vec::new();
        let tracks = midi.tracks();

        let ticks_per_qn = midi.header().timing().ticks_per_quarter_note().unwrap();

        // is Some if the tempo is set for the whole file.
        // None if the format is sequentially independent
        let file_tempo = match midi.format_type() {
            FormatType::SequentiallyIndependent => None,
            FormatType::Simultaneous | FormatType::SingleMultiChannel => {
                let first_track = tracks.first().unwrap();
                Some(first_track.info().tempo)
            }
        };

        for track in tracks {
            let track_tempo = file_tempo.unwrap_or(track.info().tempo);
            let micros_per_quarter_note = track_tempo.micros_per_quarter_note();

            let (micros_per_tick, offset_in_micros) = match midi.header().timing() {
                Timing::Smpte(v) => {
                    //µs_per_tick = 1 000 000 / (fps × ticks_per_frame)
                    //FPS is −24/−25/−29/−30 in the high byte of division;
                    // ticks per frame is the low byte.

                    let frames_per_second = v.fps().as_division() as u32;
                    let ticks_per_frame = v.ticks_per_frame() as u32;
                    let ticks_per_second = frames_per_second * ticks_per_frame;
                    let micros_per_tick = 1_000_000. / ticks_per_second as f64;

                    //NOTE: if the file header uses smpte, that overrides any track smpte offset.
                    let offset_in_micros = track
                        .info()
                        .smpte_offset
                        .as_ref()
                        .map(|offset| {
                            if offset.fps != v.fps() {
                                warn!(
                                    "Header's fps({}) does not align with track's fps({}). \
                                    The file's fps will override the track's!",
                                    v.fps().as_f64(),
                                    offset.fps.as_f64()
                                );
                            }
                            offset.as_micros_with_override(v.fps())
                        })
                        .unwrap_or(0.);

                    (micros_per_tick, offset_in_micros)
                }
                Timing::TicksPerQuarterNote(tpqn) => {
                    // µs_per_tick = tempo_meta / TPQN
                    // micro_seconds/quarternote * quarternote_per_tick (1/ticks per qn)
                    let micros_per_tick =
                        micros_per_quarter_note as f64 / tpqn.ticks_per_quarter_note() as f64;

                    let offset_in_micros = track
                        .info()
                        .smpte_offset
                        .as_ref()
                        .map(|offset| offset.as_micros())
                        .unwrap_or(0.);

                    (micros_per_tick, offset_in_micros)
                }
            };

            for event in track.events() {
                match event.event() {
                    LiveEvent::ChannelVoice(cv) => {
                        let tick = event.accumulated_ticks();
                        let micros = micros_per_tick * tick as f64;

                        commands.push(Timed::new(micros as u64, *cv));
                    }
                    _ => {
                        //idk
                    }
                }
            }
        }
        MidiSong::new(commands)
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
