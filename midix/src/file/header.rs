use crate::prelude::*;

#[doc = r#"
   Information about the timing of the MIDI file
"#]
pub struct Header {
    timing: Timing,
}

impl Header {
    /// Create a new header from timing
    pub fn new(timing: Timing) -> Self {
        Self { timing }
    }
    /// Get the timing props
    pub fn timing(&self) -> &Timing {
        &self.timing
    }
}
