use crate::prelude::*;
use std::io::ErrorKind;

pub trait SystemCommonMessageTrait {
    fn status(&self) -> u8;
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum SystemCommonMessage<'a> {
    /// A system-exclusive event.
    ///
    /// System Exclusive events start with a `0xF0` byte and finish with a `0xF7` byte, but this
    /// vector does not include either: it only includes data bytes in the `0x00..=0x7F` range.
    SystemExclusive(SysEx<'a>),
    /*/// A MIDI Time Code Quarter Frame message, carrying a tag type and a 4-bit tag value.
    MidiTimeCodeQuarterFrame {
        message: MtcQuarterFrameMessage,
        tag: u8,
    },*/
    /// An undefined System Common message
    Undefined(u8),
    /// The number of MIDI beats (6 x MIDI clocks) that have elapsed since the start of the
    /// sequence.
    SongPositionPointer { lsb: u8, msb: u8 },
    /// Select a given song index.
    SongSelect(u8),
    /// Request the device to tune itself.
    TuneRequest,
}
impl SystemCommonMessage<'_> {
    fn status(&self) -> u8 {
        use SystemCommonMessage::*;
        match self {
            SystemExclusive(_) => 0xF0,
            SongPositionPointer { .. } => 0xF2,
            SongSelect(_) => 0xF3,
            TuneRequest => 0xF6,
            Undefined(v) => *v,
        }
    }
}

impl AsMidiBytes for SystemCommonMessage<'_> {
    fn as_bytes(&self) -> Vec<u8> {
        use SystemCommonMessage::*;
        match self {
            SystemExclusive(b) => b.as_bytes(),
            SongPositionPointer { lsb, msb } => {
                vec![self.status(), *lsb, *msb]
            }
            SongSelect(v) => vec![self.status(), *v],
            TuneRequest => vec![self.status()],
            Undefined(_) => vec![self.status()],
        }
    }
}

impl FromMidiMessage for SystemCommonMessage<'_> {
    const MIN_STATUS_BYTE: u8 = 0xF0;
    const MAX_STATUS_BYTE: u8 = 0xF7;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error> {
        let ev = match status {
            0xF0 => {
                //SysEx
                let data = data
                    .iter()
                    .copied()
                    .take_while(|byte| byte != &0xF7)
                    .collect::<Vec<_>>();
                SystemCommonMessage::SystemExclusive(SysEx::new(data))
            }
            /*0xF1 if data.len() >= 1 => {
                //MTC Quarter Frame
                SystemCommonMessage::MidiTimeCodeQuarterFrame {
                    message: MtcQuarterFrameMessage::from_bits(data[0] >> 4).unwrap(),
                    tag: data[0] & 0b0000_1111,
                }
            }*/
            0xF2 if data.len() == 2 => {
                //Song Position
                SystemCommonMessage::SongPositionPointer {
                    lsb: data[0],
                    msb: data[1],
                }
            }
            0xF3 if data.len() == 1 => {
                //Song Select
                SystemCommonMessage::SongSelect(data[0])
            }
            0xF6 => {
                //Tune Request
                SystemCommonMessage::TuneRequest
            }
            0xF1..=0xF5 if data.is_empty() => {
                //Unknown system common event
                SystemCommonMessage::Undefined(status)
            }
            _ => {
                //Invalid/Unknown/Unreachable event
                //(Including F7 SysEx End Marker)
                return Err(io_error!(
                    ErrorKind::InvalidInput,
                    "Could not read System Common Message"
                ));
            }
        };
        Ok(ev)
    }
}

/// The different kinds of info a Midi Time Code Quarter Frame message can carry.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MtcQuarterFrameMessage {
    /// The low nibble of the frame count.
    FramesLow,
    /// The high nibble of the frame count.
    FramesHigh,
    /// The low nibble of the second count.
    SecondsLow,
    /// The high nibble of the second count.
    SecondsHigh,
    /// The low nibble of the minute count.
    MinutesLow,
    /// The high nibble of the minute count.
    MinutesHigh,
    /// The low nibble of the hour count.
    HoursLow,
    /// The high nibble of the hour count.
    HoursHigh,
}

impl MidiBits for MtcQuarterFrameMessage {
    type BitRepresentation = u8;
    fn as_bits(&self) -> u8 {
        use MtcQuarterFrameMessage::*;
        match self {
            FramesLow => 0,
            FramesHigh => 1,
            SecondsLow => 2,
            SecondsHigh => 3,
            MinutesLow => 4,
            MinutesHigh => 5,
            HoursLow => 6,
            HoursHigh => 7,
        }
    }
    fn from_bits(code: u8) -> Result<MtcQuarterFrameMessage, std::io::Error> {
        use MtcQuarterFrameMessage::*;
        Ok(match code {
            0 => FramesLow,
            1 => FramesHigh,
            2 => SecondsLow,
            3 => SecondsHigh,
            4 => MinutesLow,
            5 => MinutesHigh,
            6 => HoursLow,
            7 => HoursHigh,
            _ => {
                return Err(io_error!(
                    ErrorKind::InvalidData,
                    "Invalid MtcQuarterFrameMessage"
                ))
            }
        })
    }
}

/// Borrowed bytes from some reader. EXPECT THIS TO BREAK IN A FUTURE RELEASE!
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum BorrowedSystemCommonMessage<'a> {
    /// A system-exclusive event.
    ///
    /// System Exclusive events start with a `0xF0` byte and finish with a `0xF7` byte, but this
    /// vector does not include either: it only includes data bytes in the `0x00..=0x7F` range.
    SystemExclusive(&'a [u8]),
    /*/// A MIDI Time Code Quarter Frame message, carrying a tag type and a 4-bit tag value.
    MidiTimeCodeQuarterFrame {
        message: MtcQuarterFrameMessage,
        tag: u8,
    },*/
    /// An undefined System Common message
    Undefined(u8),
    /// The number of MIDI beats (6 x MIDI clocks) that have elapsed since the start of the
    /// sequence.
    SongPositionPointer { lsb: u8, msb: u8 },
    /// Select a given song index.
    SongSelect(u8),
    /// Request the device to tune itself.
    TuneRequest,
}

impl SystemCommonMessageTrait for BorrowedSystemCommonMessage<'_> {
    fn status(&self) -> u8 {
        use BorrowedSystemCommonMessage::*;
        match &self {
            SystemExclusive(_) => 0xF0,
            SongPositionPointer { .. } => 0xF2,
            SongSelect(_) => 0xF3,
            TuneRequest => 0xF6,
            Undefined(v) => *v,
        }
    }
}
