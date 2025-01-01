#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ParseState {
    /// Initial state in which reader stay after creation. Transition from that
    /// state could produce a `HeaderChunk` or `TrackChunk` event. The next
    /// state is always `InsideMidi`. The reader will never return to this state. The
    /// event emitted during transition to `InsideMarkup` is a `HeaderChunk` if the
    /// first symbol not `<`, otherwise no event are emitted.
    Init,
    /// Bytes have been read in the midi file, and outside of a track's contents
    InsideMidi,
    /// The cursor is pointing to events inside the track
    InsideTrack {
        start: usize,
        length: usize,
        prev_status: Option<u8>,
    },
    /// Reader enters this state when `Eof` event generated or an error occurred.
    /// This is the last state, the reader stay in it forever.
    Done,
}

#[derive(Clone, Debug)]
pub(super) struct ReaderState {
    offset: usize,
    last_error_offset: usize,
    state: ParseState,
}

#[allow(dead_code)]
impl ReaderState {
    #[must_use]
    pub const fn default() -> Self {
        Self {
            offset: 0,
            last_error_offset: 0,
            state: ParseState::Init,
        }
    }
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[must_use]
    pub fn parse_state(&self) -> ParseState {
        self.state
    }
    pub fn parse_state_mut(&mut self) -> &mut ParseState {
        &mut self.state
    }
    pub fn set_parse_state(&mut self, state: ParseState) {
        self.state = state;
    }

    pub const fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
    pub const fn increment_offset(&mut self, amt: usize) {
        self.offset += amt;
    }
    pub const fn decrement_offset(&mut self, amt: usize) {
        self.offset -= amt;
    }
    pub const fn set_last_error_offset(&mut self, offset: usize) {
        self.last_error_offset = offset;
    }
    pub const fn increment_last_error_offset(&mut self, offset: usize) {
        self.last_error_offset += offset;
    }
}
