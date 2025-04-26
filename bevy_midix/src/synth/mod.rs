#![doc = r#"
Synthesizer resources, setup and plugins
"#]

use crate::{
    prelude::SoundFont,
    song::{SongId, SongWriter},
};
use bevy::prelude::*;
use crossbeam_channel::{SendError, Sender};
use midix::prelude::{ChannelVoiceMessage, Timed};
use std::sync::Mutex;
use thiserror::Error;
use tinyaudio::OutputDevice;

mod plugin;
pub use plugin::*;

mod sink;
pub(crate) use sink::*;

enum SynthState {
    NotLoaded,
    LoadHandle {
        sound_font: Handle<SoundFont>,
    },
    Loaded {
        synth_channel: Sender<ChannelVoiceMessage>,
        /// the sink channel will process delayed events and interface with the synth channel directly
        sink_channel: Sender<SinkCommand>,
    },
}

/// Errors related to synthing
#[derive(Error, Debug)]
pub enum SynthError {
    /// The synthesizer isn't ready yet (soundfont not loaded)
    #[error("The synthesizer isn't ready yet (soundfont not loaded)")]
    NotReady,
    /// Disconnected from sink. This is usually because the thread panicked somehow.
    ///
    /// If this is unexpected, please file an issue with logs!
    #[error("The sink has disconnected")]
    SinkDisconnected,
    /// Disconnected from synth. This is usually because the thread panicked somehow.
    ///
    /// If this is unexpected, please file an issue with logs!
    #[error("The synth has disconnected")]
    SynthDisconnected,
}

impl From<SendError<SinkCommand>> for SynthError {
    fn from(_value: SendError<SinkCommand>) -> Self {
        Self::SinkDisconnected
    }
}

impl From<SendError<ChannelVoiceMessage>> for SynthError {
    fn from(_value: SendError<ChannelVoiceMessage>) -> Self {
        Self::SynthDisconnected
    }
}

/// Plays audio commands with the provided soundfont
///
/// Pass the synth midi events via the `Synth::handle_event` method
///
/// see [`ChannelVoiceMessage`] for the list of options
#[derive(Resource)]
pub struct Synth {
    params: SynthParams,
    synthesizer: SynthState,
    _device: Option<Mutex<OutputDevice>>,
}

impl Synth {
    /// Create a new synth given the following parameters:
    ///
    /// 1. The number of output channels
    ///
    /// A good default is 2? I actually don't know
    ///
    /// 2. The sampling rate for the audio font (if this needs more info, please file an issue for docs)
    ///
    /// A good default is 44100
    ///
    /// 3. The sample count for each channel
    ///
    /// A good default is 441
    pub fn new(params: SynthParams) -> Self {
        Self {
            params,
            ..Default::default()
        }
    }

    /// Send an event for the synth to play instantly
    ///
    /// # Errors
    ///
    /// If the synth is not ready for commands. See [`Synth::is_ready`]
    pub fn push_event(&self, event: ChannelVoiceMessage) -> Result<(), SynthError> {
        let SynthState::Loaded { synth_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return Err(SynthError::NotReady);
        };
        synth_channel.send(event)?;
        Ok(())
    }
    /// Send a single event for the synth to play instantly
    ///
    /// # Errors
    ///
    /// If the synth is not ready for commands. See [`Synth::is_ready`]
    pub fn push_timed_event(&self, event: Timed<ChannelVoiceMessage>) -> Result<(), SynthError> {
        let SynthState::Loaded { sink_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return Err(SynthError::NotReady);
        };
        sink_channel.send(SinkCommand::PlayEvent(event))?;
        Ok(())
    }

    /// Push something that makes the synth do things.
    ///
    /// Returns a songid IF it already has one, or IF one was generated (because of looping)
    ///
    /// # Errors
    ///
    /// If the synth is not ready for commands. See [`Synth::is_ready`]
    pub fn push_audio(&self, song: impl SongWriter) -> Result<Option<SongId>, SynthError> {
        let SynthState::Loaded { sink_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return Err(SynthError::NotReady);
        };
        let (id, song_type) = match (song.song_id(), song.looped()) {
            (Some(id), _) => (
                Some(id),
                SongType::Identified {
                    id,
                    looped: song.looped(),
                },
            ),
            (None, true) => {
                let id = SongId::default();
                (Some(id), SongType::Identified { id, looped: true })
            }
            _ => (None, SongType::Anonymous),
        };

        sink_channel.send(SinkCommand::NewSong {
            song_type,
            commands: song.events().collect(),
        })?;
        Ok(id)
    }

    /// Stop a certain song from playing.
    ///
    /// If stop_voices is false, any currently playing notes will continue to be held.
    ///
    /// # Errors
    ///
    /// If the synth is not ready for commands. See [`Synth::is_ready`]
    pub fn stop(&self, song_id: SongId, stop_voices: bool) -> Result<(), SynthError> {
        let SynthState::Loaded { sink_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return Err(SynthError::NotReady);
        };
        sink_channel.send(SinkCommand::Stop {
            song_id: Some(song_id),
            stop_voices,
        })?;
        Ok(())
    }

    /// Returns true if the sound font has been loaded!
    pub fn is_ready(&self) -> bool {
        matches!(self.synthesizer, SynthState::Loaded { .. })
    }

    /// Provide a handle to the soundfont file
    pub fn use_soundfont(&mut self, sound_font: Handle<SoundFont>) {
        self.synthesizer = SynthState::LoadHandle { sound_font };
        self._device = None;
    }
}

impl Default for Synth {
    fn default() -> Self {
        Self {
            params: SynthParams::default(),
            synthesizer: SynthState::NotLoaded,
            _device: None,
        }
    }
}
