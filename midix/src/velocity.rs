use crate::prelude::*;
use core::fmt;

/// Identifies the velocity of a key press, or a key unpress, or an aftertouch.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Velocity(DataByte);

impl Velocity {
    /// Returns a max velocity
    pub const MAX: Velocity = Velocity(DataByte::new_unchecked(127));
    /// Returns a velocity of zero.
    pub const ZERO: Velocity = Velocity(DataByte::new_unchecked(0));

    /// Creates a new velocity from the provided byte
    ///
    /// Checks for correctness (leading 0 bit)
    pub fn new<B>(rep: B) -> Result<Self, ParseError>
    where
        B: TryInto<DataByte, Error = ParseError>,
    {
        rep.try_into().map(Self)
    }

    /// Creates a new velocity without checking the bytes' validity.
    pub const fn new_unchecked(byte: u8) -> Self {
        Self(DataByte::new_unchecked(byte))
    }

    /// Get a reference to the underlying byte
    pub const fn byte(&self) -> u8 {
        self.0.0
    }

    /// Get the dynamic of the velocity...fortississississimo
    pub const fn dynamic(&self) -> Dynamic {
        match self.byte() {
            0 => Dynamic::off(),
            1..16 => Dynamic::ppp(),
            16..32 => Dynamic::pp(),
            32..48 => Dynamic::p(),
            48..64 => Dynamic::mp(),
            64..80 => Dynamic::mf(),
            80..96 => Dynamic::f(),
            96..112 => Dynamic::ff(),
            _ => Dynamic::fff(),
        }
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The musical analog of the digital velocity
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dynamic {
    /// no sound
    Off,
    /// very quiet
    Pianississimo,
    /// pretty quiet
    Pianissimo,
    /// quiet
    Piano,
    /// kinda quiet
    MezzoPiano,
    /// kinda loud
    MezzoForte,
    /// loud
    Forte,
    /// pretty loud
    Fortissimo,
    /// very loud
    Fortississimo,
}

impl fmt::Display for Dynamic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Dynamic::*;
        match self {
            Off => write!(f, "off"),
            Pianissimo => write!(f, "pp"),
            Piano => write!(f, "p"),
            Pianississimo => write!(f, "ppp"),
            MezzoPiano => write!(f, "mp"),
            MezzoForte => write!(f, "mf"),
            Forte => write!(f, "f"),
            Fortissimo => write!(f, "ff"),
            Fortississimo => write!(f, "fff"),
        }
    }
}

impl Dynamic {
    /// No sound
    pub const fn off() -> Self {
        Self::Off
    }

    /// very quiet
    pub const fn ppp() -> Self {
        Self::Pianississimo
    }

    /// pretty quiet
    pub const fn pp() -> Self {
        Self::Pianissimo
    }

    /// quiet
    pub const fn p() -> Self {
        Self::Piano
    }

    /// kinda quiet
    pub const fn mp() -> Self {
        Self::MezzoPiano
    }

    /// kinda loud
    pub const fn mf() -> Self {
        Self::MezzoForte
    }

    /// loud
    pub const fn f() -> Self {
        Self::Forte
    }

    /// pretty loud
    pub const fn ff() -> Self {
        Self::Fortissimo
    }

    /// very loud
    pub const fn fff() -> Self {
        Self::Fortississimo
    }
}
