/// (in microseconds per MIDI quarter-note)
///
/// FF 51 03 tttttt
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Tempo(u32);

impl Default for Tempo {
    fn default() -> Self {
        Self(500000)
    }
}

impl Tempo {
    /// Interprete a byte slice as a tempo
    pub fn new_from_bytes(v: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(v))
    }

    /// The count of microseconds per midi quarter-note
    pub fn micros_per_quarter_note(&self) -> u32 {
        self.0
    }
}

#[test]
fn known_tempo() {
    let tempo = [0x07, 0xA1, 0x20];

    let tempo = Tempo::new_from_bytes(tempo);

    assert_eq!(tempo.micros_per_quarter_note(), 500000);
}
