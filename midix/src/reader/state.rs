#[derive(Clone, Debug)]
pub struct ReaderState {
    offset: u64,
    last_error_offset: u64,
}

impl ReaderState {
    pub(crate) const fn default() -> Self {
        Self {
            offset: 0,
            last_error_offset: 0,
        }
    }
    pub(crate) const fn offset(&self) -> u64 {
        self.offset
    }
}
