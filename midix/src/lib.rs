#![cfg_attr(feature = "nightly", feature(const_for))]
#![doc = r#"
# MIDIx

A crate used to parse MIDI events

Docs still need to be written. For now, follow the reference of our fork (`midly`).

A few differences:
LiveEvent -> MidiMessage
MidiMessage types have been divided into three different enumerations.


## Acknowledgments
A lot of the documentation is copied directly from <http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html> .

This document was originally distributed in text format by The International MIDI Association.

© Copyright 1999 David Back.

EMail: david@csw2.co.uk

Web: http://www.csw2.co.uk

The documentation states "This document may be freely copied in whole or in part provided the copy contains this Acknowledgement."

Please give it a look for a deeper dive into MIDI!
"#]

#[macro_use]
mod error;

pub mod bytes;
pub mod channel;
pub mod file;
pub mod live;
pub mod message;
pub mod parser;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::bytes::*;
    pub use crate::channel::Channel;
    pub use crate::file::{builder::*, chunk::*, format::*, header::*, meta::*, track::*};
    pub use crate::live::*;
    pub use crate::message::{controller::*, key::*, pitch_bend::*, program::*, velocity::*, *};
    pub use crate::parser::{
        reader::*,
        types::{chunk::*, header::*, track::*, *},
        *,
    };

    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
