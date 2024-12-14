use crate::ChannelVoiceMessage;

/// System Realtime messages are one-byte messages that only occur within live MIDI streams.
/// They are usually time-sensitive, get top priority and can even be transmitted in between other
/// messages.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SystemRealtime {
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
impl SystemRealtime {
    /// Create a system realtime event from its id byte.
    #[inline]
    pub fn new(status: u8) -> SystemRealtime {
        use SystemRealtime::*;
        match status {
            0xF8 => TimingClock,
            0xFA => Start,
            0xFB => Continue,
            0xFC => Stop,
            0xFE => ActiveSensing,
            0xFF => Reset,
            _ => {
                //Unknown system realtime event
                Undefined(status)
            }
        }
    }

    /// Get the id byte for this system realtime message.
    #[inline]
    pub fn encode(self) -> u8 {
        use SystemRealtime::*;
        match self {
            TimingClock => 0xF8,
            Start => 0xFA,
            Continue => 0xFB,
            Stop => 0xFC,
            ActiveSensing => 0xFE,
            Reset => 0xFF,
            Undefined(byte) => byte,
        }
    }
}
