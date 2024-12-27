use super::{HeaderChunk, TrackChunk, TrackEvent};

#[derive(Debug, PartialEq)]
pub enum Event<'a> {
    Header(HeaderChunk<'a>),
    Track(TrackChunk),
    TrackEvent(TrackEvent<'a>),
    EOF,
}

impl<'a> Event<'a> {
    pub fn header(h: HeaderChunk<'a>) -> Self {
        Self::Header(h)
    }
    pub fn track(t: TrackChunk) -> Self {
        Self::Track(t)
    }
    pub fn track_event(t: TrackEvent<'a>) -> Self {
        Self::TrackEvent(t)
    }
    pub fn eof() -> Self {
        Self::EOF
    }
}
