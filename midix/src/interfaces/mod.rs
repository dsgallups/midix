#![doc = r#"

u8 to

"#]

pub trait MidiSource {
    type Event;
}

#[doc = r#"
Any type that can represent itself as MIDI

should there be an associated type? Depends on if there are
live messages vs file messages
"#]
pub trait Midi: MidiSource {}
