use crate::{prelude::*, utils};

pub struct MidiTrackEventRef<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: u32,
    event: MidiTrackMessageRef<'a>,
}

impl<'a> MidiTrackEventRef<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut OldReader<&'slc [u8]>) -> OldReadResult<Self>
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
        MidiTrackEvent::new(self.delta_time, self.event.to_owned())
    }
}
