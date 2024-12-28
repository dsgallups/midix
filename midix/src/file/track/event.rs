use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct TrackEvent<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: u32,
    event: TrackMessage<'a>,
}

impl<'a> TrackEvent<'a> {
    pub(crate) fn new(delta_time: u32, event: TrackMessage<'a>) -> Self {
        Self { delta_time, event }
    }

    pub fn delta_time(&self) -> u32 {
        self.delta_time
    }
    pub fn event(&self) -> &TrackMessage<'a> {
        &self.event
    }

    /*pub fn to_owned(self) -> MidiTrackEvent {
        MidiTrackEvent::new(self.delta_time, self.event.to_owned())
    }*/
}
