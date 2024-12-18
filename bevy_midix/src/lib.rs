#![doc = r#"
# bevy_MIDIx

Docs still need to be written. For now, follow the reference of our fork (`bevy_midi`).

A few differences:
This crate uses `midix` types.

"#]

pub mod input;
pub mod output;

pub mod prelude {
    pub use crate::{input::*, output::*, *};
    pub use midix::prelude::*;
}

pub const KEY_RANGE: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];
