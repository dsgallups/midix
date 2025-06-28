//! Firewheel-based MIDI synthesizer integration for Bevy
//!
//! This module provides a fast, component-based MIDI synthesizer using bevy_seedling
//! and firewheel audio nodes.

mod components;
mod node;
mod plugin;
mod systems;

pub use components::{MidiCommand, MidiSynthNode, MidiSynthSettings};
pub use node::{MidiSynthNodeConfig, MidiSynthProcessor};
pub use plugin::FirewheelMidiPlugin;

/// Prelude for common imports
pub mod prelude {
    pub use super::{FirewheelMidiPlugin, MidiCommand, MidiSynthNode, MidiSynthSettings};
}
