use crate::{prelude::*, utils};

/// Defines a track of a midi file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiTrack {}

// I would like to return some type of reader...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiTrackRef<'a> {
    length: &'a [u8; 4],
    data: &'a [u8],
}

impl<'a> MidiTrackRef<'a> {
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
    pub fn events(&self) -> ReadResult<Vec<MidiTrackEventRef<'a>>> {
        let mut reader = Reader::from_byte_slice(self.data);

        let mut events: Vec<MidiTrackEventRef<'a>> = Vec::new();
        loop {
            match MidiTrackEventRef::read(&mut reader) {
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessageRef<'a> {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(SystemExclusiveRef<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessageRef<'a>),
}

impl<'a> MidiTrackMessageRef<'a> {
    pub fn read<'r, 'slc>(reader: &'r mut Reader<&'slc [u8]>) -> ReadResult<Self>
    where
        'slc: 'a,
    {
        let next_event = reader.peak_next()?;

        let res = match next_event {
            0xF0 => {
                //since we've peaked
                reader.increment_buffer_position(1);
                let mut data = reader.read_varlen_slice()?;
                if !data.is_empty() {
                    //discard the last 0xF7
                    data = &data[..data.len() - 1];
                }
                Self::SystemExclusive(SystemExclusiveRef::new(data))
            }
            0xFF => {
                //since we've peaked
                reader.increment_buffer_position(1);
                Self::Meta(MetaMessageRef::read(reader)?)
            }
            _ => Self::ChannelVoice(ChannelVoiceMessage::read(reader)?),
        };

        Ok(res)
    }
}
#[test]
fn test_simple_sysex() {
    let bytes = [0xF0, 0x05, 0x43, 0x12, 0x00, 0x07, 0xF7];
    let mut reader = Reader::from_byte_slice(&bytes);
    let msg = MidiTrackMessageRef::read(&mut reader).unwrap();

    assert_eq!(
        msg,
        MidiTrackMessageRef::SystemExclusive(SystemExclusiveRef::new(&[0x43, 0x12, 0x00, 0x07]))
    );
}
