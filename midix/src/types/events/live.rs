use crate::prelude::*;
use std::io::ErrorKind;

#[doc = r"
An emittable message to/from a streaming MIDI device.

There is currently no `StreamReader` type, so this type is most often manually constructed.
"]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiveEvent<'a> {
    /// A MIDI voice message associated with a channel
    ChannelVoice(ChannelVoice<'a>),

    /// A set of common messages that are not meant to be used
    /// For input/output purposes
    SystemCommon(SystemCommon<'a>),

    /// Events that are for synchronization purposes.
    SystemRealTime(SystemRealTime),
}

impl LiveEvent<'_> {
    /// returns Some if the message contains a [`ChannelVoice`] event.
    pub fn channel_voice(&self) -> Option<&ChannelVoice<'_>> {
        match self {
            LiveEvent::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }
}

impl<'a> From<ChannelVoice<'a>> for LiveEvent<'a> {
    fn from(value: ChannelVoice<'a>) -> Self {
        Self::ChannelVoice(value)
    }
}
impl<'a> From<SystemCommon<'a>> for LiveEvent<'a> {
    fn from(value: SystemCommon<'a>) -> Self {
        Self::SystemCommon(value)
    }
}
impl From<SystemRealTime> for LiveEvent<'_> {
    fn from(value: SystemRealTime) -> Self {
        Self::SystemRealTime(value)
    }
}

impl FromMidiMessage for LiveEvent<'_> {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xFF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        match status {
            0x80..=0xEF => Ok(Self::ChannelVoice(ChannelVoice::from_status_and_data(
                status, data,
            )?)),
            0xF0..=0xF7 => Ok(Self::SystemCommon(SystemCommon::from_status_and_data(
                status, data,
            )?)),
            0xF8..=0xFF => Ok(Self::SystemRealTime(SystemRealTime::from_status_and_data(
                status, data,
            )?)),
            _ => Err(io_error!(
                ErrorKind::InvalidData,
                "Received a status that is not a midi message"
            )),
        }
    }
}

impl AsMidiBytes for LiveEvent<'_> {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            LiveEvent::ChannelVoice(c) => c.as_bytes(),
            LiveEvent::SystemCommon(s) => s.as_bytes(),
            LiveEvent::SystemRealTime(r) => r.as_bytes(),
        }
    }
}

#[test]
fn parse_note_on() {
    use crate::prelude::*;
    let message = [0b1001_0001, 0b010_01000, 0b001_00001];
    let parsed = LiveEvent::from_bytes(&message).unwrap();
    //parsed: ChannelVoice(ChannelVoiceMessage { channel: Channel(1), message: NoteOn { key: Key(72), vel: Velocity(33) } })

    assert_eq!(
        parsed,
        LiveEvent::ChannelVoice(ChannelVoice::new(
            Channel::new(1).unwrap(),
            VoiceEvent::NoteOn {
                key: Key::new(72),
                velocity: Velocity::new(33)
            }
        ))
    );
}
