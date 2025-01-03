#![doc = include_str!("../README.md")]

pub(crate) mod reader;
pub mod soundfont;
pub mod synthesizer;

pub mod prev_midifile;

pub mod prelude {
    //midifile::*,
    pub use crate::{
        soundfont::{instrument::*, preset::*, *},
        synthesizer::*,
    };

    pub(crate) use crate::{
        reader::*,
        soundfont::{generator::*, zone::*},
        /*synthesizer::{
            voice::*,
            voice::{envelope::*, region::*},
        },*/
    };
}
