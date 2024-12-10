use core::fmt;

use crate::num::u7;

/// Identifies a key press
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key(u7);

impl Key {
    /// Create a new key
    pub fn new(key: impl Into<u7>) -> Self {
        Self(key.into())
    }

    /// Writes out as int
    pub fn as_int(self) -> u8 {
        self.0.as_int()
    }

    /// Identifies the note of the key pressed
    pub fn note(self) -> Note {
        Note::from_midi_datum(self.as_int())
    }

    /// Identifies the octave of the key pressed
    pub fn octave(&self) -> Octave {
        Octave::from_midi_datum(self.as_int())
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test_note() {
    let c = Key::new(12);

    assert_eq!(Note::C, c.note());

    let a_sharp = Key::new(94);
    assert_eq!(Note::ASharp, a_sharp.note())
}

#[test]
fn test_octave() {
    let c = Key::new(12);

    assert_eq!(0, c.octave().as_number());

    let a_sharp = Key::new(94);
    assert_eq!(6, a_sharp.octave().as_number())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
/// identifies the note played
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}
impl Note {
    pub fn from_midi_datum(key: u8) -> Self {
        use Note::*;
        let note = key % 12;

        match note {
            0 => C,
            1 => CSharp,
            2 => D,
            3 => DSharp,
            4 => E,
            5 => F,
            6 => FSharp,
            7 => G,
            8 => GSharp,
            9 => A,
            10 => ASharp,
            11 => B,
            _ => unreachable!(),
        }
    }
}

pub struct Octave(i8);

impl Octave {
    pub fn from_midi_datum(key: u8) -> Self {
        let octave = key / 12;

        Self(octave as i8 - 1)
    }
    pub fn as_number(&self) -> i8 {
        self.0
    }
}
