use crate::prelude::*;

#[doc = r#"
   Information about the timing of the MIDI file
"#]
pub struct Header<'a> {
    timing: Timing<'a>,
}

impl<'a> Header<'a> {
    /// Create a new header from timing
    pub fn new(timing: Timing<'a>) -> Self {
        Self { timing }
    }
    /// Get the timing props
    pub fn timing(&self) -> &Timing<'a> {
        &self.timing
    }
}
