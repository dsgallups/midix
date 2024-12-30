use crate::prelude::*;
use std::io::ErrorKind;

#[doc = r#"
A System Common Message, used to relay data for ALL receivers, regardless of channel.
"#]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SystemCommonMessage<'a> {
    /// A system-exclusive message.
    ///
    /// System Exclusive events start with a `0xF0` byte and finish with a `0xF7` byte.
    ///
    /// Note that `SystemExclusiveMessage` is found in both [`LiveEvent`]s and [`FileEvent`]s.
    SystemExclusive(SystemExclusiveMessage<'a>),

    /// An undefined System Common message
    Undefined(StatusByte<'a>),
    /// The number of MIDI beats (6 x MIDI clocks) that have elapsed since the start of the
    /// sequence.
    SongPositionPointer(SongPositionPointer<'a>),
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
            Undefined(v) => *v.byte(),
        }
    }

    /// Represents the message as an array of bytes for some live MIDI stream
    pub fn to_bytes(&self) -> Vec<u8> {
        use SystemCommonMessage::*;
        match self {
            SystemExclusive(b) => b.to_live_bytes(),
            SongPositionPointer(spp) => {
                vec![self.status(), *spp.lsb(), *spp.msb()]
            }
            SongSelect(v) => vec![self.status(), *v],
            TuneRequest | Undefined(_) => vec![self.status()],
        }
    }
}

impl FromLiveEventBytes for SystemCommonMessage<'_> {
    const MIN_STATUS_BYTE: u8 = 0xF0;
    const MAX_STATUS_BYTE: u8 = 0xF7;
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error> {
        let ev = match status {
            0xF0 => {
                //SystemExclusiveMessage
                let data = data
                    .iter()
                    .copied()
                    .take_while(|byte| byte != &0xF7)
                    .collect::<Vec<_>>();
                SystemCommonMessage::SystemExclusive(SystemExclusiveMessage::new(data))
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
                SystemCommonMessage::SongPositionPointer(SongPositionPointer::new(
                    data[0], data[1],
                )?)
            }
            0xF3 if data.len() == 1 => {
                //Song Select
                SystemCommonMessage::SongSelect(check_u7(data[0])?)
            }
            0xF6 => {
                //Tune Request
                SystemCommonMessage::TuneRequest
            }
            0xF1..=0xF5 if data.is_empty() => {
                //Unknown system common event
                SystemCommonMessage::Undefined(StatusByte::new(status)?)
            }
            _ => {
                //Invalid/Unknown/Unreachable event
                //(Including F7 SystemExclusiveMessage End Marker)
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

impl MtcQuarterFrameMessage {
    /// Represents the message as a byte
    pub fn as_byte(&self) -> u8 {
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

    /// Creates a new message from a byte. This type always checks for correctness.
    pub fn new(code: u8) -> Result<MtcQuarterFrameMessage, std::io::Error> {
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
