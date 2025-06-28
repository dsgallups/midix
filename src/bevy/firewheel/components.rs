use crate::prelude::{ChannelVoiceMessage, SoundFont};
use bevy::prelude::*;

/// Component for sending MIDI commands to a synthesizer node
#[derive(Component, Default)]
pub struct MidiCommands {
    /// Queue of MIDI commands to send
    pub queue: Vec<ChannelVoiceMessage>,
}

impl MidiCommands {
    /// Add a MIDI command to the queue
    pub fn send(&mut self, command: ChannelVoiceMessage) {
        self.queue.push(command);
    }

    /// Add multiple MIDI commands to the queue
    pub fn send_batch(&mut self, commands: impl IntoIterator<Item = ChannelVoiceMessage>) {
        self.queue.extend(commands);
    }

    /// Take all commands, leaving the queue empty
    pub fn take(&mut self) -> Vec<ChannelVoiceMessage> {
        std::mem::take(&mut self.queue)
    }
}

/// Component that specifies which soundfont to use for a MIDI synth
#[derive(Component)]
pub struct MidiSoundfont(pub Handle<SoundFont>);

/// Optional component for MIDI synth configuration
#[derive(Component, Clone)]
pub struct MidiSynthConfig {
    /// Enable reverb effect
    pub enable_reverb: bool,
    /// Enable chorus effect
    pub enable_chorus: bool,
    /// Volume level (0.0 to 1.0)
    pub volume: f32,
}

impl Default for MidiSynthConfig {
    fn default() -> Self {
        Self {
            enable_reverb: true,
            enable_chorus: true,
            volume: 1.0,
        }
    }
}
