use crate::prelude::ChannelVoiceMessage;
use bevy::prelude::*;
use bevy_seedling::node::FirewheelNode;

/// Component that holds a reference to a MIDI synthesizer audio node
#[derive(Component)]
pub struct MidiSynthNode {
    /// The firewheel audio node handle
    pub node: Option<FirewheelNode>,
    /// Whether the node is currently active
    pub active: bool,
}

impl Default for MidiSynthNode {
    fn default() -> Self {
        Self {
            node: None,
            active: false,
        }
    }
}

/// Settings for the MIDI synthesizer
#[derive(Component, Clone)]
pub struct MidiSynthSettings {
    /// Path to the soundfont file
    pub soundfont_path: String,
    /// Sample rate for audio processing
    pub sample_rate: f32,
    /// Enable reverb effect
    pub enable_reverb: bool,
    /// Enable chorus effect
    pub enable_chorus: bool,
    /// Volume level (0.0 to 1.0)
    pub volume: f32,
}

impl Default for MidiSynthSettings {
    fn default() -> Self {
        Self {
            soundfont_path: String::new(),
            sample_rate: 44100.0,
            enable_reverb: true,
            enable_chorus: true,
            volume: 1.0,
        }
    }
}

/// Component for sending MIDI commands to a synthesizer node
#[derive(Component, Default)]
pub struct MidiCommand {
    /// Queue of MIDI commands to send
    pub commands: Vec<ChannelVoiceMessage>,
}

impl MidiCommand {
    /// Create a new MIDI command component with an empty queue
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a MIDI command to the queue
    pub fn send(&mut self, command: ChannelVoiceMessage) {
        self.commands.push(command);
    }

    /// Add multiple MIDI commands to the queue
    pub fn send_batch(&mut self, commands: impl IntoIterator<Item = ChannelVoiceMessage>) {
        self.commands.extend(commands);
    }

    /// Clear all pending commands
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Take all commands, leaving the queue empty
    pub fn take(&mut self) -> Vec<ChannelVoiceMessage> {
        std::mem::take(&mut self.commands)
    }
}

/// Marker component for entities that should output audio
#[derive(Component, Default)]
pub struct MidiAudioOutput;

/// Component that tracks the loading state of a soundfont
#[derive(Component)]
pub enum SoundfontLoadState {
    /// Not yet loaded
    NotLoaded,
    /// Currently loading
    Loading,
    /// Successfully loaded with the soundfont data
    Loaded(Vec<u8>),
    /// Failed to load with error message
    Failed(String),
}

impl Default for SoundfontLoadState {
    fn default() -> Self {
        Self::NotLoaded
    }
}
