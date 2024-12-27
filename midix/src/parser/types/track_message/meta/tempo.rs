/// (in microseconds per MIDI quarter-note)
///
/// FF 51 03 tttttt
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TempoRef<'a>(&'a [u8; 3]);

impl<'a> TempoRef<'a> {
    pub fn new(v: &'a [u8; 3]) -> Self {
        Self(v)
    }

    pub fn micros_per_quarter_note(&self) -> u32 {
        let val = [0, self.0[0], self.0[1], self.0[2]];
        u32::from_be_bytes(val)
    }
}
