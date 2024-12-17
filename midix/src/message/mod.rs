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

use crate::bytes::FromMidiMessage;

pub enum MidiMessage {
    ChannelVoiceMessage(ChannelVoiceMessage),
    SystemCommonMessage(SystemCommonMessage),
    SystemRealTimeMessage(SystemRealTimeMessage),
}

impl FromMidiMessage for MidiMessage {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xFF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        match status {
            0x80..=0xEF => Ok(Self::ChannelVoiceMessage(
                ChannelVoiceMessage::from_status_and_data(status, data)?,
            )),
            0xF0..=0xF7 => Ok(Self::SystemCommonMessage(
                SystemCommonMessage::from_status_and_data(status, data)?,
            )),
            0xF8..=0xFF => Ok(Self::SystemRealTimeMessage(
                SystemRealTimeMessage::from_status_and_data(status, data)?,
            )),
            _ => Err(io_error!(
                ErrorKind::InvalidData,
                "Received a status that is not a midi message"
            )),
        }
    }
}
