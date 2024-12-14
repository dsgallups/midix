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

use super::{SystemCommon, SystemRealtime};

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
