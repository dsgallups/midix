use super::Bytes;

#[doc = r#"
Any type that represents some midi source type.
"#]
pub trait MidiSource<'slc> {
    /// Get the max length of the source.
    fn max_len(&self) -> usize;

    /// Get the a partial slice of the source.
    fn get_slice(&self, start: usize, end: usize) -> Bytes<'slc>;

    /// Get a byte. Bytes are cheap to copy
    fn get_byte(&self, pos: usize) -> Option<u8>;
}

impl<'slc> MidiSource<'slc> for &'slc [u8] {
    fn max_len(&self) -> usize {
        self.len()
    }
    fn get_slice(&self, start: usize, end: usize) -> Bytes<'slc> {
        Bytes::new(&self[start..end])
    }
    fn get_byte(&self, pos: usize) -> Option<u8> {
        self.get(pos).copied()
    }
}
