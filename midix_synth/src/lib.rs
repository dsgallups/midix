#![doc = include_str!("../README.md")]

pub mod envelope;
pub mod generator;
pub mod instrument;
//pub mod midifile;
pub mod preset;
pub mod reader;
pub mod region;
pub mod reverb;
pub mod soundfont;
pub mod synthesizer;
pub mod voice;
pub mod zone;

pub mod prev_midifile;

pub mod prelude {
    //midifile::*,
    pub use crate::{
        envelope::*, generator::*, instrument::*, preset::*, reader::*, region::*, reverb::*,
        soundfont::*, synthesizer::*, voice::*, zone::*,
    };
}
