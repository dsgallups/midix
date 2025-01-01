use crate::prelude::TrackEvent;

#[doc = r#"
A set of track events
"#]
#[derive(Debug, Clone, PartialEq)]
pub struct Track<'a>(Vec<TrackEvent<'a>>);

impl<'a> Track<'a> {
    /// Create a new track
    pub fn new(events: Vec<TrackEvent<'a>>) -> Self {
        Self(events)
    }
}
