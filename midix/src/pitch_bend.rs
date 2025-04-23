use crate::prelude::*;

/// The value of a pitch bend, represented as 14 bits.
///
/// A value of `0x0000` indicates full bend downwards.
/// A value of `0x2000` indicates no bend.
/// A value of `0x3FFF` indicates full bend upwards.
///
/// This value is available via [`PitchBend::value`]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct PitchBend {
    lsb: DataByte,
    msb: DataByte,
}

impl PitchBend {
    /// Creates a new pitch bend given the
    /// least significant and most significant bytes.
    ///
    /// Checks for byte correctness (leading 0 bit)
    pub fn new(lsb: u8, msb: u8) -> Result<Self, ParseError> {
        let lsb = DataByte::new(lsb)?;
        let msb = DataByte::new(msb)?;
        Ok(Self { lsb, msb })
    }

    /// Creates a new pitch bend given the
    /// least significant and most significant bytes.
    ///
    /// Does not check for correctness
    pub const fn new_unchecked(lsb: u8, msb: u8) -> Self {
        Self {
            lsb: DataByte::new_unchecked(lsb),
            msb: DataByte::new_unchecked(msb),
        }
    }

    /// Returns a reference to the pitch bend's least significant byte.
    pub const fn lsb(&self) -> u8 {
        self.lsb.0
    }

    /// Returns a reference to the pitch bend's most significant byte.
    pub const fn msb(&self) -> u8 {
        self.msb.0
    }

    /// Represents a pitch bend
    pub const fn value(&self) -> u16 {
        let lsb = self.lsb.value();
        let msb = self.msb.value();
        let combined: u16 = ((msb as u16) << 8) | (lsb as u16);
        combined
    }

    /// Represents a u16, lsb then msb, as a pitch bend
    pub fn from_bits(rep: u16) -> Result<Self, ParseError> {
        let lsb = (rep >> 8) as u8;
        let msb = (rep & 0x00FF) as u8;
        Self::new(lsb, msb)
    }
    /// Represents a u16, lsb then msb, as a pitch bend.
    ///
    /// Does not check for correctness.
    pub const fn from_bits_unchecked(rep: u16) -> Self {
        let lsb = (rep >> 8) as u8;
        let msb = (rep & 0x00FF) as u8;
        Self::new_unchecked(lsb, msb)
    }
}

impl PitchBend {
    /// The minimum value of `0x0000`, indicating full bend downwards.
    pub const MIN_BYTES: u16 = 0x0000;

    /// The middle value of `0x2000`, indicating no bend.
    pub const MID_BYTES: u16 = 0x2000;

    /// The maximum value of `0x3FFF`, indicating full bend upwards.
    pub const MAX_VALUE: u16 = 0x3FFF;

    /// Create a `PitchBend` value from an int in the range `[-0x2000, 0x1FFF]`.
    ///
    /// Integers outside this range will be clamped.
    #[inline]
    pub fn from_int(int: i16) -> Self {
        PitchBend::from_bits((int.clamp(-0x2000, 0x1FFF) + 0x2000) as u16).unwrap()
    }

    /// Create a `PitchBend` value from a number in the range `[-1.0, 1.0)`.
    ///
    /// Floats outside this range will be clamped.
    #[inline]
    pub fn from_f32(float: f32) -> Self {
        PitchBend::from_int((float.clamp(-1.0, 1.0) * 0x2000 as f32) as i16)
    }

    /// Create a `PitchBend` value from a number in the range `[-1.0, 1.0)`.
    ///
    /// Floats outside this range will be clamped.
    #[inline]
    pub fn from_f64(float: f64) -> Self {
        PitchBend::from_int((float.clamp(-1.0, 1.0) * 0x2000 as f64) as i16)
    }

    /// Returns an int in the range `[-0x2000, 0x1FFF]`.
    ///
    /// Do not use this when writing to a midi file.
    #[inline]
    pub const fn as_int(self) -> i16 {
        self.value() as i16 - 0x2000
    }

    /// Returns an `f32` in the range `[-1.0, 1.0)`.
    #[inline]
    pub const fn as_f32(self) -> f32 {
        self.as_int() as f32 * (1.0 / 0x2000 as f32)
    }

    /// Returns an `f64` in the range `[-1.0, 1.0)`.
    #[inline]
    pub const fn as_f64(self) -> f64 {
        self.as_int() as f64 * (1.0 / 0x2000 as f64)
    }
}
