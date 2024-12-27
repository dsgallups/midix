use crate::prelude::*;

// I would like to return some type of reader...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiTrack<'a> {
    length: &'a [u8; 4],
}

impl<'a> MidiTrack<'a> {
    /// Assumes that the chunk type bytes ("MTrk") have ALREADY been read
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let length: &[u8; 4] = reader.read_exact_size()?;

        let read = u32::from_be_bytes(*length);

        let track_event_bytes = reader.read_exact(read as usize)?;

        Ok(Self { length })
    }
    pub fn length(&self) -> u32 {
        convert_u32(self.length)
    }
}

pub struct MidiTrackEvent<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: &'a [u8],
    event: &'a [u8],
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TrackMidiMessage<'a> {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(SystemExclusiveBorrowed<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessage<'a>),
}
