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
"#]

#[macro_use]
mod error;

pub mod bytes;
pub mod reader;
pub mod types;
pub(crate) mod utils;

pub mod prelude {
    #![doc = r#"
        Common re-exports when working with `midix`
    "#]
    pub use crate::{
        bytes::*,
        types::{
            channel::*,
            controller::*,
            events::*,
            file::{chunk::*, meta::*, track::*, *},
            key::*,
            pitch_bend::*,
            program::*,
            velocity::*,
            *,
        },
    };

    pub use crate::{
        reader::{ReadResult, Reader},
        *,
    };

    #[allow(unused_imports)]
    pub(crate) use crate::reader::{inv_data, inv_input, unexp_eof};

    pub use core::fmt::Display;

    pub(crate) use crate::utils::*;
}
