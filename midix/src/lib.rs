#![cfg_attr(feature = "nightly", feature(const_for))]
#![warn(missing_docs)]
#![doc = r#"

A high performance MIDI parser and management library.

# Getting Started

`midix` contains tools to read and write MIDI events.

TODO


# Acknowledgments
A lot of the documentation is copied directly
from [this documentation](http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html).

This reference states "This document may be freely copied in whole or in part provided the copy contains this Acknowledgement.":
```text
This document was originally distributed in text format by The International MIDI Association.
© Copyright 1999 David Back.
EMail: david@csw2.co.uk
Web: http://www.csw2.co.uk
```
Please give it a look for a deeper dive into MIDI!


Basic types that are used commonly among parsing and streaming.

# Overview

MIDI can be interpreted in two main ways: through `LiveEvent`s and regular file `Events`.

TODO
"#]

use std::io::{self, ErrorKind};

#[macro_use]
mod error;

pub mod reader;
pub(crate) mod utils;

pub mod channel;

pub mod events;
pub mod file;
mod pitch_bend;
pub use pitch_bend::*;

mod program;
pub use program::*;

mod velocity;
pub use velocity::*;

mod key;
pub use key::*;

mod channel_voice;
pub use channel_voice::*;

mod controller;
pub use controller::*;

mod common_message;
pub use common_message::*;
mod realtime_message;
pub use realtime_message::*;
mod sysex;
pub use sysex::*;

pub(crate) trait ReadDataBytesExt {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error>;
}

impl ReadDataBytesExt for &[u8] {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error> {
        self.get(byte).ok_or(io_error!(
            ErrorKind::InvalidInput,
            "Data not accessible for message!"
        ))
    }
}

pub mod prelude {
    #![doc = r#"
        Common re-exports when working with `midix`
    "#]
    pub use crate::{
        channel::*,
        events::*,
        file::{chunk::*, meta::*, track::*, *},
        *,
    };

    pub use crate::reader::{ReadResult, Reader};

    #[allow(unused_imports)]
    pub(crate) use crate::reader::{inv_data, inv_input, unexp_eof};

    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
