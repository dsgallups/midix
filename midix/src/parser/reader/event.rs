use super::HeaderChunk;

pub enum Event<'a> {
    Header(HeaderChunk<'a>),
    Track(TrackChunk<'a>),
    TrackEvent(TrackEvent<'a>),
    EOF,
}
