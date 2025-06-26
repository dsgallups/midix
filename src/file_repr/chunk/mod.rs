#![doc = r#"
Contains types for MIDI file chunks

TODO

# Overview
MIDI has two chunk types. MIDI defines anything that does
not fall into th

## [`RawHeaderChunk`]
This chunk type contains meta information about the MIDI file, such as
- [`RawFormat`](crate::prelude::RawFormat), which identifies how tracks should be played, and the claimed track count
- [`Timing`](crate::prelude::Timing), which defines how delta-seconds are to be interpreted

## [`]

"#]

mod unknown_chunk;
pub use unknown_chunk::*;

mod header;
pub use header::*;

mod track;
pub use track::*;
