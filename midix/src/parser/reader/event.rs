use super::{HeaderChunk, TrackChunk, TrackEvent};

pub enum Event<'a> {
    Header(HeaderChunk<'a>),
    Track(TrackChunk<'a>),
    TrackEvent(TrackEvent<'a>),
    EOF,
}
