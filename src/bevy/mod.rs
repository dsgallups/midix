#![warn(missing_docs)]

pub mod asset;
pub mod input;
pub mod output;
pub mod plugin;
pub mod settings;
pub mod synth;

pub mod song;

/// Commonly re-exported types
pub mod prelude {
    pub use crate::bevy::{
        asset::*, input::*, output::*, plugin::*, settings::*, song::*, synth::*,
    };
}
