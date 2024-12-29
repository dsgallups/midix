use std::borrow::Cow;

use crate::reader::{ReadResult, Reader};

#[derive(Clone, Debug, PartialEq)]
pub struct UnknownChunk<'a> {
    name: Cow<'a, [u8]>,
    inner: Cow<'a, [u8]>,
}

impl<'a> UnknownChunk<'a> {
    /// Place the bytes of an unknown chunk
    pub(crate) fn read(name: &'a [u8], reader: &mut Reader<&'a [u8]>) -> ReadResult<Self> {
        let length = u32::from_be_bytes(*reader.read_exact_size()?);
        let data = reader.read_exact(length as usize)?;
        Ok(Self {
            name: Cow::Borrowed(name),
            inner: Cow::Borrowed(data),
        })
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
