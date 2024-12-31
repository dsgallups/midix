use crate::prelude::*;

#[doc = r#"
   Information about the timing of the MIDI file
"#]
pub struct Header<'a> {
    timing: Timing<'a>,
}
