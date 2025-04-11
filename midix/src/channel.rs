#![doc = r"
# Identifier for a MIDI Channel
"]

use core::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::message::{ChannelVoiceMessage, VoiceEvent};

/// Identifies a channel for MIDI. Constructors check that the value is between 0-15.
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum Channel {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
}

impl Channel {
    // /// Identify a channel (1, 2, 3)
    // ///
    // /// # Panics
    // /// if the byte is 0
    // ///
    // /// # Errors
    // /// If the channel is greater than a value of 15
    // pub fn new(channel: u8) -> Result<Self, std::io::Error> {
    //     Ok(Self(check_u4(channel - 1)?))
    // }

    /// Send a voice event to this channel
    pub fn send_event(self, event: VoiceEvent) -> ChannelVoiceMessage {
        ChannelVoiceMessage::new(self, event)
    }

    /// Given a status byte from some [`ChannelVoiceMessage`], perform bitwise ops
    /// to get the channel
    #[must_use]
    pub fn from_status(status: u8) -> Self {
        let channel = status & 0b0000_1111;
        Channel::try_from(channel).unwrap()
    }

    /// Returns the 4-bit channel number (0-15)
    #[must_use]
    pub fn to_byte(&self) -> u8 {
        (*self).into()
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res: u8 = (*self).into();
        res.fmt(f)
    }
}

#[test]
fn channel_from_status() {
    use pretty_assertions::assert_eq;
    assert_eq!(Channel::Eight, Channel::from_status(0b1011_0111));
    assert_eq!(Channel::One, Channel::from_status(0b1011_0000));
    assert_eq!(Channel::Sixteen, Channel::from_status(0b0101_1111));
}
