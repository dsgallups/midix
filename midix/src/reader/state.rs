#[derive(Clone, Debug)]
pub struct ReaderState {
    offset: usize,
    last_error_offset: usize,
}

impl ReaderState {
    pub const fn default() -> Self {
        Self {
            offset: 0,
            last_error_offset: 0,
        }
    }
    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub const fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    pub const fn increment_offset(&mut self, amt: usize) {
        self.offset += amt;
    }
    pub const fn set_last_error_offset(&mut self, offset: usize) {
        self.last_error_offset = offset;
    }
    pub const fn increment_last_error_offset(&mut self, offset: usize) {
        self.last_error_offset += offset;
    }
}
