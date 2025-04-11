use core::fmt;
use std::io;

use crate::DataByte;

#[doc = r#"
Identifies a key for some message.

Keys are interpeted as a 7-bit number.

Each value corresponds to some [`Note`] and [`Octave`].

[`Key`] `0` is `C(-1)`, and [`Key`] `127` is `G9`.

# Example
```rust
# use midix::prelude::*;

let key_byte = 63;

let key = Key::new(key_byte).unwrap(); // 63 is between 0-127

assert_eq!(key.note(), Note::DSharp);
assert_eq!(key.octave(), Octave::new(4))
```
"#]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
pub struct Key(DataByte);

impl Key {
    /// Create a new key.
    ///
    /// Checks for correctness (leading 0 bit).
    pub fn from_databyte<B, E>(rep: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte, Error = E>,
        E: Into<io::Error>,
    {
        rep.try_into().map(Self).map_err(Into::into)
    }

    /// Create all possible keys (128)
    pub fn all() -> Vec<Self> {
        (0..128).map(|v| Key::from_databyte(v).unwrap()).collect()
    }

    /// Create a key from a given note and octave
    pub fn new(note: Note, octave: Octave) -> Self {
        let octave_byte = (octave.value() + 1) as u8;

        let key = note.get_mod_12();

        let octave_mult = (octave_byte) * 12;

        Self::from_databyte(octave_mult + key).unwrap()
    }

    /// Identifies the note of the key pressed
    #[inline]
    pub fn note(&self) -> Note {
        Note::from_data_byte(&self.0)
    }

    /// Returns true if the note of the key is sharp. Same as `is_flat`
    ///
    /// See [`Note::is_sharp`] for an example
    #[inline]
    pub fn is_sharp(&self) -> bool {
        self.note().is_sharp()
    }

    /// Returns true if the note of the key is flat. Same as `is_sharp`
    /// See [`Note::is_flat`] for an example
    #[inline]
    pub fn is_flat(&self) -> bool {
        self.note().is_flat()
    }

    /// Identifies the octave of the key pressed
    #[inline]
    pub fn octave(&self) -> Octave {
        Octave::from_data_byte(&self.0)
    }

    /// Returns the underlying byte of the key
    pub fn byte(&self) -> u8 {
        self.0 .0
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.note(), self.octave())
    }
}

#[test]
fn test_note() {
    let c = Key::from_databyte(12).unwrap();

    assert_eq!(Note::C, c.note());

    let a_sharp = Key::from_databyte(94).unwrap();
    assert_eq!(Note::ASharp, a_sharp.note());
}

#[test]
fn test_octave() {
    let c = Key::from_databyte(12).unwrap();

    assert_eq!(0, c.octave().value());

    let a_sharp = Key::from_databyte(94).unwrap();
    assert_eq!(6, a_sharp.octave().value());
}

#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[doc = r#"
Identifies some note for a [`Key`]

# Example
```rust
# use midix::prelude::*;

let note = Note::FSharp;


let key = note.with_octave(Octave::new(4));


assert_eq!(key.octave().value(), 4);
assert_eq!(key.note(), Note::FSharp);
```
"#]
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
    /// Returns an array beginning with [`Note::C`] to [`Note::B`]
    pub fn all() -> [Note; 12] {
        use Note::*;
        [C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B]
    }
    /// Create a new note from a byte with a leading 0
    ///
    /// # Errors
    /// if the byte is > 127
    pub fn new<K, E>(key: K) -> Result<Self, io::Error>
    where
        K: TryInto<DataByte, Error = E>,
        E: Into<io::Error>,
    {
        use Note::*;
        let key = key.try_into().map_err(Into::into)?;
        let note = key.value() % 12;

        Ok(match note {
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
        })
    }

    /// Returns true if the note type is sharp. Same as `is_flat`
    ///
    /// # Example
    /// ```rust
    /// # use midix::prelude::*;
    /// let note = Note::C;
    /// assert!(!note.is_sharp());
    /// let note = Note::FSharp;
    /// assert!(note.is_sharp());
    /// ```
    #[inline]
    pub fn is_sharp(&self) -> bool {
        use Note::*;
        matches!(self, CSharp | DSharp | FSharp | GSharp | ASharp)
    }

    /// Returns true if the note type is flat. Same as `is_sharp`
    ///
    /// # Example
    /// ```rust
    /// # use midix::prelude::*;
    /// let note = Note::A;
    /// assert!(!note.is_flat());
    /// let note = Note::CSharp;
    /// assert!(note.is_flat());
    /// ```
    #[inline]
    pub fn is_flat(&self) -> bool {
        self.is_sharp()
    }

    /// Identify the note from a key byte.
    #[inline]
    pub fn from_data_byte(key: &DataByte) -> Self {
        use Note::*;
        let note = key.value() % 12;

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
    fn get_mod_12(&self) -> u8 {
        use Note::*;
        match self {
            C => 0,
            CSharp => 1,
            D => 2,
            DSharp => 3,
            E => 4,
            F => 5,
            FSharp => 6,
            G => 7,
            GSharp => 8,
            A => 9,
            ASharp => 10,
            B => 11,
        }
    }

    /// Create a [`Key`] given this note and a provided [`Octave`]
    pub fn with_octave(self, octave: Octave) -> Key {
        Key::new(self, octave)
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

# Example

```rust
# use midix::prelude::*;

let octave = Octave::new(12); // clamps to 9

assert_eq!(octave.value(), 9);

let key = octave.with_note(Note::C);


assert_eq!(key.octave().value(), 9);
assert_eq!(key.note(), Note::C);
```
"#]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Octave(i8);

impl Octave {
    /// Identify an octave from a key byte.
    pub fn from_data_byte(key: &DataByte) -> Self {
        let octave = key.value() / 12;

        Self(octave as i8 - 1)
    }
    /// Should be a value between [-1, 9]. Clamps between these two values.
    pub fn new(octave: i8) -> Self {
        Self(octave.clamp(-1, 9))
    }

    /// The octave, from `[-1,9]`
    pub const fn value(&self) -> i8 {
        self.0
    }

    /// Create a [`Key`] given this octave and a provided [`Note`]
    pub fn with_note(self, note: Note) -> Key {
        Key::new(note, self)
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn key_from_note_octave_pairs() {
    for key_byte in 0..128 {
        let key = Key::from_databyte(key_byte).unwrap();

        let exp_note = key.note();
        let exp_oct = key.octave();

        let made_key = Key::new(exp_note, exp_oct);

        assert_eq!(exp_oct, made_key.octave());
        assert_eq!(exp_note, made_key.note());
        assert_eq!(key, Key::new(exp_note, exp_oct));
    }
}
