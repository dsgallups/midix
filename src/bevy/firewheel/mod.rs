//! Firewheel-based MIDI synthesizer integration for Bevy
//!
//! This module provides a fast, component-based MIDI synthesizer using bevy_seedling
//! and firewheel audio nodes. It allows you to:
//!
//! - Spawn MIDI synthesizer nodes on entities with soundfont files
//! - Send MIDI commands instantly to those nodes
//! - Hear audio output with minimal latency
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use midix::bevy::firewheel::prelude::*;
//!
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     // Load a soundfont
//!     let soundfont = asset_server.load("sounds/my_soundfont.sf2");
//!
//!     // Spawn a MIDI synthesizer
//!     let synth = commands.spawn_midi_synth(soundfont);
//! }
//!
//! fn play_note(mut query: Query<&mut MidiCommands>) {
//!     for mut commands in &mut query {
//!         // Play middle C
//!         commands.send(ChannelVoiceMessage::note_on(0, 60, 100));
//!     }
//! }
//! ```

mod components;
mod node;
mod plugin;
mod systems;

// Re-export main types
pub use components::{MidiCommands, MidiSoundfont, MidiSynthConfig, MidiSynthNode};
pub use node::{MidiNodeEvent, MidiSynthNodeConfig, MidiSynthProcessor};
pub use plugin::{FirewheelMidiPlugin, MidiCommandsExt};
pub use systems::{
    MidiInstrument, MidiSynthBundle, MidiSystemSet, debug_midi_commands, handle_note_input,
    panic_button, play_scale, set_instrument, volume_control,
};

/// Prelude for common imports
pub mod prelude {
    pub use super::{
        FirewheelMidiPlugin, MidiCommands, MidiCommandsExt, MidiInstrument, MidiSoundfont,
        MidiSynthBundle, MidiSynthConfig, MidiSynthNode, MidiSystemSet,
    };

    // Re-export ChannelVoiceMessage for convenience
    pub use crate::prelude::ChannelVoiceMessage;
}
