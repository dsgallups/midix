use reader::ReaderError;

use crate::prelude::*;

#[doc = r#"
Identifies a track chunk header. Only metadata
contained is the length, in bytes, of the
track chunk's body.

The body bytes are parsed into [`TrackEvent`]s.

"#]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackChunkHeader {
    length: u32,
}

impl TrackChunkHeader {
    /// Assumes that the chunk type bytes (`"MTrk"`) have ALREADY been read
    pub(crate) fn read<'slc, 'r, R>(reader: &'r mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
    {
        let length: BytesConst<'_, 4> = reader.read_exact_size()?;

        let length = u32::from_be_bytes(*length);

        Ok(Self { length })
    }

    /// The number of bytes proceeding the header of the track body.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.length
    }
}

#[doc = r#"
Contains the whole length of the track chunk
"#]
#[allow(dead_code)]
pub struct RawTrackChunk<'a>(Bytes<'a>);

impl<'a> RawTrackChunk<'a> {
    pub(crate) fn read<'slc, 'r, R>(reader: &'r mut Reader<R>) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
        'slc: 'a,
    {
        let length = u32::from_be_bytes(*reader.read_exact_size()?);

        let track_event_bytes = reader.read_exact(length as usize)?;

        Ok(Self(track_event_bytes))
    }

    /// Consume self to yield a list of track events
    pub fn events(self) -> ReadResult<Vec<TrackEvent<'a>>> {
        // just a guess, may over-allocate
        let mut events: Vec<TrackEvent<'a>> = Vec::with_capacity(self.0.len());
        let mut reader = Reader::from_bytes(self.0);
        let mut running_status = None;

        loop {
            match TrackEvent::read(&mut reader, &mut running_status) {
                Ok(e) => events.push(e),
                Err(err) => {
                    if err.is_eof() {
                        break;
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        Ok(events)
    }
}
