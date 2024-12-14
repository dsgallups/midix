//! Provides utilities to read and write "live" MIDI messages produced in real-time, in contrast
//! with "dead" MIDI messages as stored in a `.mid` file.
//!
//! [`LiveEvent`](enum.LiveEvent.html) is very similar to
//! [`TrackEventKind`](../enum.TrackEventKind.html), except for subtle differences such as system
//! realtime messages, which can only exist in live events, or escape sequences, which can only
//! exist within track events.
//!
//! Usually OS APIs (and notably [`midir`](https://docs.rs/midir)) produce MIDI messages as a slice
//! of raw MIDI bytes, which can be parsed through
//! [`LiveEvent::parse`](enum.LiveEvent.html#method.parse) and written through
//! [`LiveEvent::write`](enum.LiveEvent.html#method.write).
//!
//! Note that MIDI byte streams, which are not clearly delimited packets, must be parsed through
//! the [`stream`](../stream/index.html) api.

use crate::MidiMessage;

/// A live event produced by an OS API or generated on-the-fly, in contrast with "dead"
/// [`TrackEvent`](../struct.TrackEvent.html)s stored in a `.mid` file.
///
/// See the [`live`](index.html) module for more information.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum LiveEvent<'a> {
    /// A MIDI message associated with a channel, carrying musical data.
    ///
    /// Status byte in the range `0x80 ..= 0xEF`.
    Midi(MidiMessage),
    /// A System Common message, as defined by the MIDI spec, including System Exclusive events.
    ///
    /// Status byte in the range `0xF0 ..= 0xF7`.
    Common(SystemCommon<'a>),
    /// A one-byte System Realtime message.
    ///
    /// Status byte in the range `0xF8 ..= 0xFF`.
    Realtime(SystemRealtime),
}

/// A "system common event", as defined by the MIDI spec.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum SystemCommon<'a> {
    /// A system-exclusive event.
    ///
    /// System Exclusive events start with a `0xF0` byte and finish with a `0xF7` byte, but this
    /// slice does not include either: it only includes data bytes in the `0x00..=0x7F` range.
    SysEx(&'a [u8]),
    /// A MIDI Time Code Quarter Frame message, carrying a tag type and a 4-bit tag value.
    MidiTimeCodeQuarterFrame(MtcQuarterFrameMessage, u8),
    /// The number of MIDI beats (6 x MIDI clocks) that have elapsed since the start of the
    /// sequence.
    SongPosition(u16),
    /// Select a given song index.
    SongSelect(u8),
    /// Request the device to tune itself.
    TuneRequest,
    /// An undefined System Common message, with arbitrary data bytes.
    Undefined(u8, &'a [u8]),
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
    fn as_code(self) -> u8 {
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
    fn from_code(code: u8) -> Option<MtcQuarterFrameMessage> {
        use MtcQuarterFrameMessage::*;
        Some(match code {
            0 => FramesLow,
            1 => FramesHigh,
            2 => SecondsLow,
            3 => SecondsHigh,
            4 => MinutesLow,
            5 => MinutesHigh,
            6 => HoursLow,
            7 => HoursHigh,
            _ => return None,
        })
    }
}

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
