use crate::prelude::*;
use std::io::ErrorKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiLiveMessage {
    ChannelVoice(ChannelVoiceMessage),
    SystemCommon(SystemCommonMessage),
    SystemRealTime(SystemRealTimeMessage),
}

impl MidiLiveMessage {
    pub fn channel_voice(&self) -> Option<&ChannelVoiceMessage> {
        match self {
            MidiLiveMessage::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }
}

impl From<ChannelVoiceMessage> for MidiLiveMessage {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}
impl From<SystemCommonMessage> for MidiLiveMessage {
    fn from(value: SystemCommonMessage) -> Self {
        Self::SystemCommon(value)
    }
}
impl From<SystemRealTimeMessage> for MidiLiveMessage {
    fn from(value: SystemRealTimeMessage) -> Self {
        Self::SystemRealTime(value)
    }
}

impl FromMidiMessage for MidiLiveMessage {
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

impl AsMidiBytes for MidiLiveMessage {
    fn as_bytes(&self) -> Vec<u8> {
        use MidiLiveMessage::*;
        match self {
            ChannelVoice(c) => c.as_bytes(),
            SystemCommon(s) => s.as_bytes(),
            SystemRealTime(r) => r.as_bytes(),
        }
    }
}

#[test]
fn parse_note_on() {
    use crate::prelude::*;
    let message = [0b10010001, 0b01001000, 0b00100001];
    let parsed = MidiLiveMessage::from_bytes(&message).unwrap();
    //parsed: ChannelVoice(ChannelVoiceMessage { channel: Channel(1), message: NoteOn { key: Key(72), vel: Velocity(33) } })

    assert_eq!(
        parsed,
        MidiLiveMessage::ChannelVoice(ChannelVoiceMessage::new(
            Channel::new(1).unwrap(),
            ChannelVoiceEvent::NoteOn {
                key: Key::new(72),
                vel: Velocity::new(33)
            }
        ))
    );
}
