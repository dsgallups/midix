use crate::prelude::*;
use std::io::ErrorKind;

#[doc = r#"
One-byte messages that only occur in live MIDI events.

Real-Time messages are used for synchronization and are intended for all clock-
based units in a system. They contain Status bytes only — no Data bytes. Real-
Time messages may be sent at any time — even between bytes of a message which has
a different status. In such cases the Real-Time message is either acted upon or
ignored, after which the receiving process resumes under the previous status.

They are usually time-sensitive, get top priority and can even be transmitted in between other
messages.
"#]
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

impl SystemRealTimeMessage {
    /// Get the underlying byte
    pub fn byte(&self) -> u8 {
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

    /// Interpret a byte as a [`SystemRealTimeMessage`]
    pub fn from_byte(rep: u8) -> Result<Self, std::io::Error> {
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

impl FromLiveEventBytes for SystemRealTimeMessage {
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
        Self::from_byte(status)
    }
}
