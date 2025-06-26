use alloc::borrow::Cow;

use crate::reader::{MidiSource, ReadResult, Reader};

#[doc = r#"
Identifies a chunk of a MIDI file that cannot be parsed.

It is up to the user for how to handle such chunks. Typically, these data are
ignored. However, sometimes there are particular use cases for handling
non-standard chunk types. We leave the option up to you.
"#]
#[derive(Clone, Debug, PartialEq)]
pub struct UnknownChunk<'a> {
    name: Cow<'a, [u8]>,
    inner: Cow<'a, [u8]>,
}

impl<'a> UnknownChunk<'a> {
    /// Place the bytes of an unknown chunk
    pub(crate) fn read<'slc, 'r, R>(
        name: Cow<'a, [u8]>,
        reader: &'r mut Reader<R>,
    ) -> ReadResult<Self>
    where
        R: MidiSource<'slc>,
        'slc: 'a,
    {
        let length = u32::from_be_bytes(reader.read_exact_size()?);
        let data = reader.read_exact(length as usize)?;
        Ok(Self { name, inner: data })
    }

    /// Get the length of the unknown chunk
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true when the bytes identified by the chunk is of length 0
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
