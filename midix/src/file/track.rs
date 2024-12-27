use crate::prelude::*;

// I would like to return some type of reader...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiTrack<'a> {
    length: &'a [u8; 4],
    data: &'a [u8],
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

        Ok(Self {
            length,
            data: track_event_bytes,
        })
    }
    pub fn length(&self) -> u32 {
        convert_u32(self.length)
    }
    /// Slow, can be improved by implementing iterator on reader
    pub fn events(&self) -> ReadResult<Vec<MidiTrackEvent<'a>>> {
        let mut reader = Reader::from_byte_slice(self.data);

        let mut events: Vec<MidiTrackEvent<'a>> = Vec::new();
        loop {
            match MidiTrackEvent::read(&mut reader) {
                Ok(e) => events.push(e),
                Err(err) => match err {
                    ReaderError::EndOfReader => break,
                    e => return Err(e),
                },
            }
        }

        Ok(events)
    }
}

pub struct MidiTrackEvent<'a> {
    /// Variable length quantity
    /// Delta-time is in some fraction of a beat
    /// (or a second, for recording a track with SMPTE times),
    /// as specified in the header chunk.
    delta_time: &'a [u8],
    event: MidiTrackMessage<'a>,
}

impl<'a> MidiTrackEvent<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let delta_time = reader.read_next();
        todo!();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessage<'a> {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(SystemExclusiveBorrowed<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessage<'a>),
}

impl<'a> MidiTrackMessage<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        todo!();
    }
}
