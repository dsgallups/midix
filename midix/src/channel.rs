#![doc = r"
# Identifier for a MIDI Channel
"]

use core::fmt;
use std::borrow::Cow;

use crate::{
    message::{ChannelVoiceMessage, VoiceEvent},
    utils::check_u4,
};

/// Identifies a channel for MIDI. Constructors check that the value is between 0-15.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ChannelId<'a>(Cow<'a, u8>);

impl<'a> ChannelId<'a> {
    /// Identify a channel (1, 2, 3)
    ///
    /// # Panics
    /// if the byte is 0
    ///
    /// # Errors
    /// If the channel is greater than a value of 15
    pub fn new(channel: u8) -> Result<Self, std::io::Error> {
        Ok(Self(Cow::Owned(check_u4(channel - 1)?)))
    }

    /// Send a voice event to this channel
    pub fn send_event(self, event: VoiceEvent<'a>) -> ChannelVoiceMessage<'a> {
        ChannelVoiceMessage::new(self, event)
    }
    /// returns 1-16
    pub fn value(&self) -> u8 {
        *self.0 + 1
    }

    /// Identify a channel (1, 2, 3)
    ///
    /// Does not check for correctness
    pub fn new_unchecked(channel: u8) -> Self {
        Self(Cow::Owned(channel))
    }

    /// Identify a channel from a borrowed value (&1, &2, &3)
    pub fn new_borrowed_unchecked(channel: &'a u8) -> Self {
        Self(Cow::Borrowed(channel))
    }

    /// Given a status byte from some [`ChannelVoiceMessage`], perform bitwise ops
    /// to get the channel
    #[must_use]
    pub fn from_status(status: u8) -> Self {
        let channel = status & 0b0000_1111;
        Self(Cow::Owned(channel))
    }

    /// Returns the 4-bit channel number (0-15)
    #[must_use]
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for ChannelId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
