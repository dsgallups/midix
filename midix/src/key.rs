use core::fmt;
use std::borrow::Cow;

use crate::utils::check_u7;

#[doc = r#"
Identifies a key for some message.

Keys are interpeted as a 7-bit number.

Each value corresponds to some [`Note`] and [`Octave`].

[`Key`] `0` is `C(-1)`, and [`Key`] `127` is `G9`.
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Key<'a>(Cow<'a, u8>);

impl<'a> Key<'a> {
    /// Create a new key.
    ///
    /// Checks for correctness (leading 0 bit).
    pub fn new(rep: u8) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        Ok(Self(Cow::Owned(check_u7(rep)?)))
    }

    /// Create a new key. Does not check for correctness.
    pub const fn new_unchecked(key: u8) -> Self {
        Self(Cow::Owned(key))
    }
    /// Create a new key. Does not check for correctness.
    pub const fn new_borrowed_unchecked(key: &'a u8) -> Self {
        Self(Cow::Borrowed(key))
    }

    /// Identifies the note of the key pressed
    #[inline]
    pub fn note(&self) -> Note {
        Note::from_key_byte(*self.0)
    }

    /// Identifies the octave of the key pressed
    #[inline]
    pub fn octave(&self) -> Octave {
        Octave::from_key_byte(*self.0)
    }

    /// Returns the underlying byte of the key
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.note(), self.octave())
    }
}

#[test]
fn test_note() {
    let c = Key::new_unchecked(12);

    assert_eq!(Note::C, c.note());

    let a_sharp = Key::new_unchecked(94);
    assert_eq!(Note::ASharp, a_sharp.note());
}

#[test]
fn test_octave() {
    let c = Key::new_unchecked(12);

    assert_eq!(0, c.octave().as_number());

    let a_sharp = Key::new_unchecked(94);
    assert_eq!(6, a_sharp.octave().as_number());
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
/// Identifier for the note played
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
    /// Identify the note from a key byte.
    pub const fn from_key_byte(key: u8) -> Self {
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

#[doc = r#"
Identifies the octave for a [`Key`]. Values range from -1 to 9.
"#]
pub struct Octave(i8);

impl Octave {
    /// Identify an octave from a key byte.
    pub const fn from_key_byte(key: u8) -> Self {
        let octave = key / 12;

        Self(octave as i8 - 1)
    }
    /// The octave, from `[-1,9]`
    pub const fn as_number(&self) -> i8 {
        self.0
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
