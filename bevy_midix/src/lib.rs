pub mod input;
pub mod output;

pub mod prelude {
    pub use crate::{input::*, output::*, *};
    pub use midix::prelude::*;
}

pub const KEY_RANGE: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];
