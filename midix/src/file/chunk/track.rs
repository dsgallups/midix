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
    pub(crate) fn read(reader: &mut Reader<&[u8]>) -> ReadResult<Self> {
        let length: &[u8; 4] = reader.read_exact_size()?;

        let length = u32::from_be_bytes(*length);

        Ok(Self { length })
    }

    /// The number of bytes proceeding the header of the track body.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.length
    }
}
