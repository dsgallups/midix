#![doc = r"
# Identifier for a MIDI Channel
"]

use core::fmt;

use crate::{
    message::{ChannelVoiceMessage, VoiceEvent},
    utils::check_u4,
};

/// Identifies a channel for MIDI. Constructors check that the value is between 0-15.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ChannelId(u8);

impl ChannelId {
    /// Identify a channel (1, 2, 3)
    ///
    /// # Panics
    /// if the byte is 0
    ///
    /// # Errors
    /// If the channel is greater than a value of 15
    pub fn new(channel: u8) -> Result<Self, std::io::Error> {
        Ok(Self(check_u4(channel - 1)?))
    }

    /// Send a voice event to this channel
    pub fn send_event(self, event: VoiceEvent) -> ChannelVoiceMessage {
        ChannelVoiceMessage::new(self, event)
    }
    /// returns 1-16
    pub fn value(&self) -> u8 {
        self.0 + 1
    }

    /// Identify a channel (0, 1, 2)
    ///
    /// Does not check for correctness
    pub fn new_unchecked(channel: u8) -> Self {
        Self(channel)
    }

    /// Given a status byte from some [`ChannelVoiceMessage`], perform bitwise ops
    /// to get the channel
    #[must_use]
    pub fn from_status(status: u8) -> Self {
        let channel = status & 0b0000_1111;
        Self(channel)
    }

    /// Returns the 4-bit channel number (0-15)
    #[must_use]
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0 + 1).fmt(f)
    }
}
