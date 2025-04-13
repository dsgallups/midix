#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod asset;
pub mod input;
pub mod output;
pub mod plugin;
mod settings;
pub mod synth;
pub use settings::*;

/// Re-export of [`midix`]
pub mod midix {
    pub use midix::*;
}

/// Commonly re-exported types
pub mod prelude {
    pub use crate::MidiSettings;
    pub use crate::{asset::*, input::*, output::*, plugin::*, synth::*};
}
