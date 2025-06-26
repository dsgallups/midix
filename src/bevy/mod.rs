#![warn(missing_docs)]

pub mod asset;
pub mod input;
pub mod output;
pub mod plugin;
mod settings;
pub mod synth;
pub use settings::*;

pub mod song;

/// Commonly re-exported types
pub mod prelude {
    pub use crate::bevy::MidiSettings;
    pub use crate::bevy::{asset::*, input::*, output::*, plugin::*, song::*, synth::*};
}
