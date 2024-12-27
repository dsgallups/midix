#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiTiming {
    TicksPerQuarterNote(u16),
}
