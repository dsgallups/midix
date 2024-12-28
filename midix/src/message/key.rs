use core::fmt;
use std::borrow::Cow;

use crate::bytes::MidiBits;
/*
/// Identifies a key press
///
/// TODO docs
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key(u8);

impl MidiBits for Key {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(check_u7(rep)?))
    }
}

impl Key {
    /// Create a new key. Does not check for correctness.
    pub const fn new(key: u8) -> Self {
        Self(key)
    }

    /// Identifies the note of the key pressed
    pub const fn note(self) -> Note {
        Note::from_midi_datum(self.0)
    }

    /// Identifies the octave of the key pressed
    pub const fn octave(&self) -> Octave {
        Octave::from_midi_datum(self.0)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.note(), self.octave())
    }
}*/

/// Identifies a key press
///
/// TODO docs
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key<'a>(Cow<'a, u8>);

impl MidiBits for Key<'_> {
    type BitRepresentation = u8;
    fn as_bits(&self) -> Self::BitRepresentation {
        *self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(Cow::Owned(rep)))
    }
}

impl<'a> Key<'a> {
    pub const fn new(key: u8) -> Self {
        Self(Cow::Owned(key))
    }
    /// Create a new key. Does not check for u7.
    pub(crate) const fn new_borrowed(key: &'a u8) -> Self {
        Self(Cow::Borrowed(key))
    }

    /// Identifies the note of the key pressed
    #[inline]
    pub fn note(&self) -> Note {
        Note::from_midi_datum(*self.0)
    }

    /// Identifies the octave of the key pressed
    #[inline]
    pub fn octave(&self) -> Octave {
        Octave::from_midi_datum(*self.0)
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.note(), self.octave())
    }
}

#[test]
fn test_note() {
    let c = Key::new(12);

    assert_eq!(Note::C, c.note());

    let a_sharp = Key::new(94);
    assert_eq!(Note::ASharp, a_sharp.note());
}

#[test]
fn test_octave() {
    let c = Key::new(12);

    assert_eq!(0, c.octave().as_number());

    let a_sharp = Key::new(94);
    assert_eq!(6, a_sharp.octave().as_number());
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
    pub const fn from_midi_datum(key: u8) -> Self {
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
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Note::*;
        match self {
            C => write!(f, "C"),
            CSharp => write!(f, "C#/Db"),
            D => write!(f, "D"),
            DSharp => write!(f, "D#/Eb"),
            E => write!(f, "E"),
            F => write!(f, "F"),
            FSharp => write!(f, "F#/Gb"),
            G => write!(f, "G"),
            GSharp => write!(f, "G#Ab"),
            A => write!(f, "A"),
            ASharp => write!(f, "A#/Bb"),
            B => write!(f, "B"),
        }
    }
}

pub struct Octave(i8);

impl Octave {
    pub const fn from_midi_datum(key: u8) -> Self {
        let octave = key / 12;

        Self(octave as i8 - 1)
    }
    pub const fn as_number(&self) -> i8 {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
