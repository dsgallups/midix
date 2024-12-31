pub trait MidiSource<'slc> {
    fn max_len(&self) -> usize;

    fn get_slice(&self, start: usize, end: usize) -> &'slc [u8];

    fn get_byte(&self, pos: usize) -> Option<&'slc u8>;
}

impl<'slc> MidiSource<'slc> for &'slc [u8] {
    fn max_len(&self) -> usize {
        self.len()
    }
    fn get_slice(&self, start: usize, end: usize) -> &'slc [u8] {
        &self[start..end]
    }
    fn get_byte(&self, pos: usize) -> Option<&'slc u8> {
        self.get(pos)
    }
}
