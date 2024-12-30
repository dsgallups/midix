use std::fmt::Debug;

use crate::prelude::*;

#[doc = r#"
Identifies some event emitted by a track in a MIDI file.

# Overview
All MIDI track events have an associated `delta_time`. This
identifies the amount of time since the previous event.

"#]
#[derive(Clone, PartialEq)]
pub struct TrackEvent<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: u32,
    event: TrackMessage<'a>,
}

impl Debug for TrackEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Track Event {{ delta_time: 0x{:02X}, event: {:?} }}",
            self.delta_time, self.event
        )
    }
}

impl<'a> TrackEvent<'a> {
    pub(crate) fn new(delta_time: u32, event: TrackMessage<'a>) -> Self {
        Self { delta_time, event }
    }

    /// Get the difference in time from the last event
    ///
    /// The actual value should be interpreted by the MIDI file's
    /// [`Timing`] event.
    pub fn delta_time(&self) -> u32 {
        self.delta_time
    }
    /// Get a refrence to the message for the track event
    pub fn event(&self) -> &TrackMessage<'a> {
        &self.event
    }
}
