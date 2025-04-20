#![doc = r"
# Identifier for a MIDI Channel
"]

use core::fmt;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::message::{ChannelVoiceMessage, VoiceEvent};

/// Identifies a channel for MIDI.
///
/// To get this channel from a `u8`, use [`Channel::try_from_primitive`].
#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive, PartialOrd, Ord,
)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
#[repr(u8)]
pub enum Channel {
    /// 0bxxxx0000
    One = 0,
    /// 0bxxxx0001
    Two,
    /// 0bxxxx0010
    Three,
    /// 0bxxxx0011
    Four,
    /// 0bxxxx0100
    Five,
    /// 0bxxxx0101
    Six,
    /// 0bxxxx0110
    Seven,
    /// 0bxxxx0111
    Eight,
    /// 0bxxxx1000
    Nine,
    /// 0bxxxx1001
    ///
    /// Note: MIDI gives Channel Ten a special role (drums).
    ///
    /// Therefore, this channel may have different properties than you would expect!
    Ten,
    /// 0bxxxx1010
    Eleven,
    /// 0bxxxx1011
    Twelve,
    /// 0bxxxx1100
    Thirteen,
    /// 0bxxxx1101
    Fourteen,
    /// 0bxxxx1110
    Fifteen,
    /// 0bxxxx1111
    Sixteen,
}

impl Channel {
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
