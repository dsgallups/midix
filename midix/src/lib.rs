#![cfg_attr(feature = "nightly", feature(const_for))]
#![warn(missing_docs)]
#![doc = r#"
A high performance MIDI reader.

# Overview

`midix` provides a min-copy parser ([`Reader`]) to read events from `.mid` files.
Additionally, `midix` provides the user with [`LiveEvent::from_bytes`](crate::events::LiveEvent), which will parse
events from a live MIDI source.



# Getting Started

MIDI can be interpreted in two main ways: through `LiveEvent`s and regular file `Events`.

# Example
To read from a file, use the [`Reader`](crate::prelude::Reader):
```rust
use midix::prelude::*;

let midi_header = [
    /* MIDI Header */
    0x4D, 0x54, 0x68, 0x64, // "MThd"
    0x00, 0x00, 0x00, 0x06, // Chunk length (6)
    0x00, 0x00, // format 0
    0x00, 0x01, // one track
    0x00, 0x60  // 96 per quarter note
];

let mut reader = Reader::from_byte_slice(&midi_header);

// The first and only event will be the midi header
let Ok(FileEvent::Header(header)) = reader.read_event() else {
    panic!("Expected a header event");
};

// format 0 implies a single multi-channel file (only one track)
assert_eq!(header.format_type(), FormatType::SingleMultiChannel);

assert_eq!(
    header.timing().ticks_per_quarter_note(),
    Timing::new_ticks_from_byte_slice(&[0, 96]).ticks_per_quarter_note()
);

```
To parse a live event

```rust
use midix::prelude::*;

/* Ch.3 Note On C4, forte */
let note_on = [0x92, 0x3C, 0x60];

// NoteOn is a channel voice message
// Alternatively, use VoiceEvent::read_bytes(&note_on)
let Ok(LiveEvent::ChannelVoice(channel_voice_msg)) = LiveEvent::from_bytes(&note_on) else {
    panic!("Expected a channel voice event");
};

let VoiceEvent::NoteOn { key, velocity } = channel_voice_msg.event() else {
    panic!("Expected a note on event");
};

assert_eq!(channel_voice_msg.channel(), Channel::new(3).unwrap());
assert_eq!(key.note(), Note::C);
assert_eq!(key.octave().as_number(), 4);
assert_eq!(velocity.value(), 96);
```


# Semantic Versioning and Support
`midix` will adhere to semantic versioning.

`midix` is not feature complete. expect new features in minor changes.

midix will NOT break its interfaces without a major revision, which may happen with some frequency.

## General feature schedule
The SUPPORT.md file denotes the length of time major revisions are supported.

When the major version of the crate is incremented, new features for the previous version(s)
will likely be neglected. If you need a feature for an older version before the end
of its maintenence period, please let me know!

# Acknowledgments
A lot of the documentation is copied directly from
[this documentation](http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html).

This reference states "This document may be freely copied in whole or in part provided the copy contains this Acknowledgement.":
```text
This document was originally distributed in text format by The International MIDI Association.
© Copyright 1999 David Back.
EMail: david@csw2.co.uk
Web: http://www.csw2.co.uk
```
Please give it a look for a deeper dive into MIDI!
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

mod system_common;
pub use system_common::*;

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
