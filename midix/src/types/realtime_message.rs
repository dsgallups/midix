use crate::prelude::*;
use std::io::ErrorKind;

/// System Realtime messages are one-byte messages that only occur within live MIDI streams.
/// They are usually time-sensitive, get top priority and can even be transmitted in between other
/// messages.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SystemRealTimeMessage {
    /// If sent, they should be sent 24 times per quarter note.
    TimingClock,
    /// Request the device to start playing at position 0.
    Start,
    /// Request the device to continue playing without resetting the position.
    Continue,
    /// Request the device to stop playing, but keep track of the position where it stopped.
    Stop,
    /// Once one of these messages is transmitted, a message should arrive every 300ms or else the
    /// connection is considered broken.
    ActiveSensing,
    /// Request the device to reset itself, usually to the same state as it was after turning on.
    /// Usually, turns off all playing notes, clears running status, sets song position to 0, etc...
    Reset,
    /// An unknown system realtime message, with the given id byte.
    Undefined(u8),
}

impl AsMidiBytes for SystemRealTimeMessage {
    fn as_bytes(&self) -> Vec<u8> {
        vec![self.as_bits()]
    }
}

impl FromMidiMessage for SystemRealTimeMessage {
    const MIN_STATUS_BYTE: u8 = 0xF8;
    const MAX_STATUS_BYTE: u8 = 0xFF;

    /// Create a system realtime event from its id byte.
    fn from_status_and_data(status: u8, bytes: &[u8]) -> Result<Self, std::io::Error> {
        if bytes.is_empty() {
            return Err(io_error!(
                ErrorKind::InvalidData,
                "System real time messages do not have data bytes"
            ));
        }
        Self::from_bits(status)
    }
}
impl MidiBits for SystemRealTimeMessage {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        use SystemRealTimeMessage::*;
        match self {
            TimingClock => 0xF8,
            Undefined(s) => *s,
            Start => 0xFA,
            Continue => 0xFB,
            Stop => 0xFC,
            ActiveSensing => 0xFE,
            Reset => 0xFF,
        }
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        use SystemRealTimeMessage::*;
        Ok(match rep {
            0xF8 => TimingClock,
            0xFA => Start,
            0xFB => Continue,
            0xFC => Stop,
            0xFE => ActiveSensing,
            0xFF => Reset,
            _ => {
                //Unknown system realtime event
                Undefined(rep)
            }
        })
    }
}
