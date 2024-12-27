mod timing;
pub use timing::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiHeader {
    timing: MidiTiming,
}
impl MidiHeader {
    pub fn new(timing: MidiTiming) -> Self {
        Self { timing }
    }
}
