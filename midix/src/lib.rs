#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All of the errors this crate produces.
#[macro_use]
mod error;

pub mod bytes;
mod channel;
pub mod event;
pub mod message;
mod primitive;
pub(crate) mod utils;

pub use crate::{
    channel::Channel,
    event::{MetaMessage, TrackEvent, TrackEventKind},
    message::{ChannelVoiceEvent, ChannelVoiceMessage},
    primitive::{Format, Fps, SmpteTime, Timing},
};
