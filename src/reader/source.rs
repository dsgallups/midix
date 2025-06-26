use alloc::borrow::Cow;

#[doc = r#"
Any type that represents some midi source type.
"#]
pub trait MidiSource<'slc> {
    /// Get the max length of the source.
    fn max_len(&self) -> usize;

    /// Get the a partial slice of the source.
    fn get_slice(&self, start: usize, end: usize) -> Option<Cow<'slc, [u8]>>;

    /// Get a byte. Cow are cheap to copy
    fn get_byte(&self, pos: usize) -> Option<u8>;
}

impl<'slc> MidiSource<'slc> for &'slc [u8] {
    fn max_len(&self) -> usize {
        self.len()
    }
    fn get_slice(&self, start: usize, end: usize) -> Option<Cow<'slc, [u8]>> {
        self.get(start..end).map(Into::into)
    }
    fn get_byte(&self, pos: usize) -> Option<u8> {
        self.get(pos).copied()
    }
}

impl<'a> MidiSource<'a> for Cow<'a, [u8]> {
    fn max_len(&self) -> usize {
        self.len()
    }

    fn get_slice(&self, start: usize, end: usize) -> Option<Cow<'a, [u8]>> {
        match self {
            Cow::Borrowed(v) => v.get(start..end).map(Into::into),
            Cow::Owned(v) => {
                let slice = v.get(start..end)?;
                Some(slice.to_vec().into())
            }
        }
    }
    fn get_byte(&self, pos: usize) -> Option<u8> {
        self.get(pos).copied()
    }
}
