/// (in microseconds per MIDI quarter-note)
///
/// FF 51 03 tttttt
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TempoRef<'a>(&'a [u8; 3]);

impl<'a> TempoRef<'a> {
    pub fn new(v: &'a [u8; 3]) -> Self {
        Self(v)
    }
}
