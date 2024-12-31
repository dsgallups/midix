use crate::prelude::TrackMessage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track<'a>(Vec<TrackMessage<'a>>);
