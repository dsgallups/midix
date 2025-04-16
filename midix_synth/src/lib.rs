#![doc = include_str!("../README.md")]

pub(crate) mod reader;
pub mod soundfont;
pub mod synthesizer;

mod utils;

pub mod prelude {
    pub use crate::{
        soundfont::{instrument::*, preset::*, *},
        synthesizer::*,
    };

    pub(crate) use crate::{
        reader::*,
        soundfont::{generator::*, zone::*},
    };
}
