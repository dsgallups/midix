#![doc = r#"
# MIDIx

A crate used to parse MIDI events

Docs still need to be written. For now, follow the reference of our fork (`midly`).

A few differences:
LiveEvent -> MidiMessage
MidiMessage types have been divided into three different enumerations.

"#]

#[macro_use]
mod error;

pub mod bytes;
pub mod channel;
pub mod message;
pub mod reader;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::bytes::*;
    pub use crate::channel::Channel;
    pub use crate::message::{controller::*, key::*, pitch_bend::*, program::*, velocity::*, *};
    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
