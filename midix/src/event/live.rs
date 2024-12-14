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

use std::io::ErrorKind;

use crate::{bytes::FromMidiMessage, ChannelVoiceMessage};

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
    ChannelVoice(ChannelVoiceMessage),
    /// A System Common message, as defined by the MIDI spec, including System Exclusive events.
    ///
    /// Status byte in the range `0xF0 ..= 0xF7`.
    Common(SystemCommon<'a>),
    /// A one-byte System Realtime message.
    ///
    /// Status byte in the range `0xF8 ..= 0xFF`.
    Realtime(SystemRealtime),
}

impl<'a> FromMidiMessage for LiveEvent<'a> {
    const MIN_STATUS_BYTE: u8 = 0b1000000;
    const MAX_STATUS_BYTE: u8 = 0b1111111;

    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        match status {
            0x80..=0xEF => {
                // Channel Voice message
                Ok(Self::ChannelVoice(
                    ChannelVoiceMessage::from_status_and_data(status, data)?,
                ))
            }
            0xF8..=0xFF => {
                // System Realtime
                let ev = SystemRealtime::new(status);
                Ok(LiveEvent::Realtime(ev))
            }
            _ => {
                // System Common
                let ev = SystemCommon::read(status, data)?;
                Ok(LiveEvent::Common(ev))
            }
        };
        todo!()
    }
}
