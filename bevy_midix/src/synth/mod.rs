#![doc = r#"
Synthesizer resources, setup and plugins
"#]

use crate::prelude::SoundFont;
use bevy::prelude::*;
use crossbeam_channel::Sender;
use midix::prelude::ChannelVoiceMessage;
use std::sync::Mutex;
use tinyaudio::OutputDevice;

mod plugin;
pub use plugin::*;

mod sink;
pub use sink::*;

enum SynthState {
    NotLoaded,
    LoadHandle {
        sound_font: Handle<SoundFont>,
    },
    Loaded {
        synth_channel: Sender<ChannelVoiceMessage>,
        /// the sink channel will process delayed events and interface with the synth channel directly
        sink_channel: Sender<MidiSong>,
    },
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
    pub fn handle_event(&self, event: ChannelVoiceMessage) {
        let SynthState::Loaded { synth_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return;
        };
        synth_channel.send(event).unwrap();
    }

    /// Push something that makes the synth do things
    pub fn push_audio(&self, song: &impl MidiCommandSource) {
        let SynthState::Loaded { sink_channel, .. } = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return;
        };
        let commands = song.to_commands();
        sink_channel.send(commands).unwrap();
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

/// This defines a song, a file, or otherwise
/// that has timestamps associated with midi events.
///
/// this is named as such not to conflict with [`midix::MidiSource`]
pub trait MidiCommandSource {
    /// Create sink commands this type.
    fn to_commands(&self) -> MidiSong;
}
