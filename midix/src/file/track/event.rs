use crate::prelude::*;

use super::utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiTrackEvent {
    delta_time: u32,
    event: MidiTrackMessage,
}

pub struct MidiTrackEventRef<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: u32,
    event: MidiTrackMessageRef<'a>,
}

impl<'a> MidiTrackEventRef<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let delta_time = utils::decode_varlen(reader)?;

        let event = MidiTrackMessageRef::read(reader)?;
        Ok(Self { delta_time, event })
    }
    pub fn delta_time(&self) -> u32 {
        self.delta_time
    }
    pub fn event(&self) -> &MidiTrackMessageRef {
        &self.event
    }

    pub fn to_owned(self) -> MidiTrackEvent {
        MidiTrackEvent {
            delta_time: self.delta_time,
            event: self.event.to_owned(),
        }
    }
}
