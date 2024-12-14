#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All of the errors this crate produces.
#[macro_use]
mod error;

pub mod bytes;
mod channel;
pub mod event;
pub mod live;
mod message;
mod primitive;
pub(crate) mod utils;

pub mod midly {
    pub use midly::*;
}

pub use crate::{
    channel::Channel,
    error::{Error, ErrorKind, Result},
    event::{MetaMessage, TrackEvent, TrackEventKind},
    message::{ChannelVoiceEvent, ChannelVoiceMessage},
    primitive::{Format, Fps, SmpteTime, Timing},
};
