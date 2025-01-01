use crate::prelude::{TrackEvent, TrackMessage};

#[derive(Debug, Clone, PartialEq)]
pub struct Track<'a>(Vec<TrackEvent<'a>>);

impl<'a> Track<'a> {
    pub fn new(events: Vec<TrackEvent<'a>>) -> Self {
        Self(events)
    }
}
