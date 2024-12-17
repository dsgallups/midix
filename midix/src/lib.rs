#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// All of the errors this crate produces.
#[macro_use]
mod error;

pub mod bytes;
mod channel;
pub mod message;
pub(crate) mod utils;

pub use crate::{
    channel::Channel,
    message::{ChannelVoiceEvent, ChannelVoiceMessage},
};
