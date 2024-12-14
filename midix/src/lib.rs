#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All of the errors this crate produces.
#[macro_use]
mod error;

mod channel;
mod event;
mod key;
pub mod live;
mod message;
mod pitch_bend;
mod primitive;
mod velocity;

pub mod midly {
    pub use midly::*;
}

pub use crate::{
    channel::Channel,
    error::{Error, ErrorKind, Result},
    event::{MetaMessage, TrackEvent, TrackEventKind},
    key::Key,
    message::{MidiEvent, MidiMessage},
    pitch_bend::PitchBend,
    primitive::{Format, Fps, SmpteTime, Timing},
    velocity::Velocity,
};
