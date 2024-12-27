use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiTrackEvent {
    delta_time: u32,
    event: MidiTrackMessage,
}

impl MidiTrackEvent {
    pub fn new(delta_time: u32, event: MidiTrackMessage) -> Self {
        Self { delta_time, event }
    }
}
