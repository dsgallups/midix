#![doc = r#"
Rusty representation of a [`MidiFile`]

TODO
"#]

pub mod format;
use format::Format;
pub mod track;

#[doc = r#"
TODO
"#]
pub struct MidiFile<'a> {
    format: Format<'a>,
}
