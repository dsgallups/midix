#![doc = r#"
Basic types that are used commonly among parsing and streaming.

# Overview

MIDI can be interpreted in two main ways: through `LiveEvent`s and regular file `Events`.

TODO


"#]

pub mod channel;
pub mod controller;
pub mod events;
pub mod file;
pub mod key;
pub mod pitch_bend;
pub mod program;
pub mod velocity;

mod channel_voice;
pub use channel_voice::*;

mod common_message;
pub use common_message::*;
mod realtime_message;
pub use realtime_message::*;
mod sysex;
pub use sysex::*;
