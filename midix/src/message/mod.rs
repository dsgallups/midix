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

use crate::prelude::*;

pub trait MidiMessage {
    fn channel_voice(&self) -> Option<&ChannelVoiceMessage>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OwnedMidiMessage {
    ChannelVoice(ChannelVoiceMessage),
    SystemCommon(OwnedSystemCommonMessage),
    SystemRealTime(SystemRealTimeMessage),
}

impl From<ChannelVoiceMessage> for OwnedMidiMessage {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}
impl From<OwnedSystemCommonMessage> for OwnedMidiMessage {
    fn from(value: OwnedSystemCommonMessage) -> Self {
        Self::SystemCommon(value)
    }
}
impl From<SystemRealTimeMessage> for OwnedMidiMessage {
    fn from(value: SystemRealTimeMessage) -> Self {
        Self::SystemRealTime(value)
    }
}

impl MidiMessage for OwnedMidiMessage {
    fn channel_voice(&self) -> Option<&ChannelVoiceMessage> {
        match self {
            OwnedMidiMessage::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }
}

impl FromMidiMessage for OwnedMidiMessage {
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
                OwnedSystemCommonMessage::from_status_and_data(status, data)?,
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

impl AsMidiBytes for OwnedMidiMessage {
    fn as_bytes(&self) -> Vec<u8> {
        use OwnedMidiMessage::*;
        match self {
            ChannelVoice(c) => c.as_bytes(),
            SystemCommon(s) => s.as_bytes(),
            SystemRealTime(r) => r.as_bytes(),
        }
    }
}

#[test]
fn parse_note_on() {
    let message = [0b1001_0001, 0b01001000, 0b00100001];
    let parsed = OwnedMidiMessage::from_bytes(&message).unwrap();
    //parsed: ChannelVoice(ChannelVoiceMessage { channel: Channel(1), message: NoteOn { key: Key(72), vel: Velocity(33) } })

    assert_eq!(
        parsed,
        OwnedMidiMessage::ChannelVoice(ChannelVoiceMessage::new(
            Channel::new(1).unwrap(),
            ChannelVoiceEvent::NoteOn {
                key: Key::new(72),
                vel: Velocity::new(33)
            }
        ))
    );
}

/// Borrowed bytes from a reader. EXPECT THIS TO CHANGE IN A FUTURE RELEASE!
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BorrowedMidiMessage<'a> {
    ChannelVoice(ChannelVoiceMessage),
    SystemCommon(BorrowedSystemCommonMessage<'a>),
    SystemRealTime(SystemRealTimeMessage),
}

impl MidiMessage for BorrowedMidiMessage<'_> {
    fn channel_voice(&self) -> Option<&ChannelVoiceMessage> {
        match self {
            BorrowedMidiMessage::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }
}
