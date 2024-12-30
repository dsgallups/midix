use std::borrow::Cow;

/// (in microseconds per MIDI quarter-note)
///
/// FF 51 03 tttttt
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Tempo<'a>(Cow<'a, [u8; 3]>);

impl<'a> Tempo<'a> {
    /// Interprete a byte slice as a tempo
    pub fn new_from_byte_slice(v: &'a [u8; 3]) -> Self {
        Self(Cow::Borrowed(v))
    }

    /// The count of microseconds per midi quarter-note
    pub fn micros_per_quarter_note(&self) -> u32 {
        let val = [0, self.0[0], self.0[1], self.0[2]];
        u32::from_be_bytes(val)
    }
}
