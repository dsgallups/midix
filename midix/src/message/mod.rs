pub mod controller;
pub mod key;
pub mod pitch_bend;
pub mod program;
pub mod velocity;
mod voice_message;
use std::io::ErrorKind;

pub use voice_message::*;
mod common_message;
pub use common_message::*;
mod realtime_message;
pub use realtime_message::*;
mod mtcquarterframe;

use crate::bytes::{AsMidiBytes, FromMidiMessage};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiMessage {
    ChannelVoice(ChannelVoiceMessage),
    SystemCommon(SystemCommonMessage),
    SystemRealTime(SystemRealTimeMessage),
}

impl From<ChannelVoiceMessage> for MidiMessage {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}
impl From<SystemCommonMessage> for MidiMessage {
    fn from(value: SystemCommonMessage) -> Self {
        Self::SystemCommon(value)
    }
}
impl From<SystemRealTimeMessage> for MidiMessage {
    fn from(value: SystemRealTimeMessage) -> Self {
        Self::SystemRealTime(value)
    }
}

impl MidiMessage {
    pub fn channel_voice(&self) -> Option<&ChannelVoiceMessage> {
        match self {
            MidiMessage::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }
}

impl FromMidiMessage for MidiMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xFF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        match status {
            0x80..=0xEF => Ok(Self::ChannelVoice(
                ChannelVoiceMessage::from_status_and_data(status, data)?,
            )),
            0xF0..=0xF7 => Ok(Self::SystemCommon(
                SystemCommonMessage::from_status_and_data(status, data)?,
            )),
            0xF8..=0xFF => Ok(Self::SystemRealTime(
                SystemRealTimeMessage::from_status_and_data(status, data)?,
            )),
            _ => Err(io_error!(
                ErrorKind::InvalidData,
                "Received a status that is not a midi message"
            )),
        }
    }
}

impl AsMidiBytes for MidiMessage {
    fn as_bytes(&self) -> Vec<u8> {
        use MidiMessage::*;
        match self {
            ChannelVoice(c) => c.as_bytes(),
            SystemCommon(s) => s.as_bytes(),
            SystemRealTime(r) => r.as_bytes(),
        }
    }
}
