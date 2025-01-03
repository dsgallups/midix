#![doc = r#"
Contains byte-analogous representations to raw MIDI files

These types are distinct from [`MidiFile`](crate::prelude::MidiFile),
as MidiFile interprets these types and fundamentally restructures the data.
"#]

//pub mod builder;
mod format;
pub use format::*;
pub mod chunk;
pub mod meta;
pub mod track;

/// Represents a 4 character type
///
/// Each chunk has a 4-character type and a 32-bit length,
/// which is the number of bytes in the chunk. This structure allows
/// future chunk types to be designed which may be easily be ignored
/// if encountered by a program written before the chunk type is introduced.
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum MidiChunkType {
    /// Represents the byte length of the midi header.
    ///
    /// Begins with `"MThd"`
    Header,
    /// Represents the byte length of a midi track
    ///
    /// Begins with `"MTrk"`
    Track,
    /// A chunk type that is not known by this crate
    Unknown,
}
