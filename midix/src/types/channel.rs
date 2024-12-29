#![doc = r"
# Identifier for a MIDI Channel
"]

use core::fmt;
use std::borrow::Cow;

use crate::utils::check_u4;

/// Identifies a channel for MIDI. Constructors check that the value is between 0-15.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Channel<'a>(Cow<'a, u8>);

impl<'a> Channel<'a> {
    /// Identify a channel (1, 2, 3)
    /// # Errors
    /// If the channel is greater than a value of 15
    pub fn new(channel: u8) -> Result<Self, std::io::Error> {
        Ok(Self(Cow::Owned(check_u4(channel)?)))
    }

    /// Identify a channel from a borrowed value (&1, &2, &3)
    /// # Errors
    /// If the channel is greater than a value of 15
    pub fn new_borrowed(channel: &'a u8) -> Result<Self, std::io::Error> {
        Ok(Self(Cow::Borrowed(channel)))
    }

    /// Given a status byte from some [`ChannelVoice`] event, perform bitwise ops
    /// to get the channel
    #[must_use]
    pub fn from_status(status: u8) -> Self {
        let channel = status & 0b0000_1111;
        Self(Cow::Owned(channel + 1))
    }

    /// Returns the 4-bit channel number
    #[must_use]
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for Channel<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
