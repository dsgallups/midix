use crate::prelude::*;

/// Identifies something that can be interpreted from the bytes of a live MIDI stream
pub trait FromLiveEventBytes {
    /// The minimum allowed status byte for the type
    const MIN_STATUS_BYTE: u8;

    /// The maximum allowed status byte for the type
    const MAX_STATUS_BYTE: u8;

    /// Attempt to create the type from a byte slice
    ///
    /// # Errors
    /// If the byte slice cannot actually represent the type
    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        if bytes.is_empty() {
            return Err(ParseError::InvalidLength(0));
        }
        let (status, data) = bytes.split_at(1);
        let status = status[0];
        if !(Self::MIN_STATUS_BYTE..=Self::MAX_STATUS_BYTE).contains(&status) {
            return Err(ParseError::InvalidStatusByte(status));
        }

        Self::from_status_and_data(status, data)
    }
    /// Attempt to create the type from a status and set of data.
    ///
    /// This is used mainly for comfority in [`ChannelVoiceMessage`](crate::prelude::ChannelVoiceMessage)s.
    ///
    /// # Errors
    /// If the status and data cannot represent the type
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[doc = r"
An emittable message to/from a streaming MIDI device.

There is currently no `StreamReader` type, so this type is most often manually constructed.
"]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiveEvent<'a> {
    /// A MIDI voice message associated with a channel
    ChannelVoice(ChannelVoiceMessage),

    /// A set of common messages that are not meant to be used
    /// For input/output purposes
    SysCommon(SystemCommonMessage<'a>),

    /// Events that are for synchronization purposes.
    SysRealTime(SystemRealTimeMessage),
}

impl LiveEvent<'_> {
    /// returns Some if the message is a [`ChannelVoiceMessage`].
    pub fn channel_voice(&self) -> Option<&ChannelVoiceMessage> {
        match self {
            LiveEvent::ChannelVoice(c) => Some(c),
            _ => None,
        }
    }

    // /// Returns the event as a set of bytes. These bytes are to be interpreted by a MIDI live stream
    // pub fn to_bytes(&self) -> Vec<u8> {
    //     match self {
    //         LiveEvent::ChannelVoice(c) => c.to_bytes(),
    //         LiveEvent::SysCommon(s) => s.to_bytes(),
    //         LiveEvent::SysRealTime(r) => vec![r.byte()],
    //     }
    // }
}

impl From<ChannelVoiceMessage> for LiveEvent<'_> {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}
impl<'a> From<SystemCommonMessage<'a>> for LiveEvent<'a> {
    fn from(value: SystemCommonMessage<'a>) -> Self {
        Self::SysCommon(value)
    }
}

impl<'a> From<SystemExclusiveMessage<'a>> for LiveEvent<'a> {
    fn from(value: SystemExclusiveMessage<'a>) -> Self {
        Self::SysCommon(SystemCommonMessage::SystemExclusive(value))
    }
}
impl From<SystemRealTimeMessage> for LiveEvent<'_> {
    fn from(value: SystemRealTimeMessage) -> Self {
        Self::SysRealTime(value)
    }
}

impl FromLiveEventBytes for LiveEvent<'_> {
    const MIN_STATUS_BYTE: u8 = 0x80;
    const MAX_STATUS_BYTE: u8 = 0xFF;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        match status {
            0x80..=0xEF => Ok(Self::ChannelVoice(
                ChannelVoiceMessage::from_status_and_data(status, data)?,
            )),
            0xF0..=0xF7 => Ok(Self::SysCommon(SystemCommonMessage::from_status_and_data(
                status, data,
            )?)),
            0xF8..=0xFF => Ok(Self::SysRealTime(
                SystemRealTimeMessage::from_status_and_data(status, data)?,
            )),
            b => Err(ParseError::InvalidStatusByte(b)),
        }
    }
}

// #[test]
// fn parse_note_on() {
//     use crate::prelude::*;
//     let message = [0b1001_0001, 0b0100_1000, 0b001_00001];
//     let parsed = LiveEvent::from_bytes(&message).unwrap();
//     //parsed: ChannelVoice(ChannelVoiceMessage { channel: Channel(1), message: NoteOn { key: Key(72), vel: Velocity(33) } })

//     assert_eq!(
//         parsed,
//         LiveEvent::ChannelVoice(ChannelVoiceMessage::new(
//             Channel::Two,
//             VoiceEvent::NoteOn {
//                 key: Key::from_databyte(72).unwrap(),
//                 velocity: Velocity::new(33).unwrap()
//             }
//         ))
//     );
// }
