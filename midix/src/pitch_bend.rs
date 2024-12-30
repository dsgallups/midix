use std::borrow::Cow;

use crate::prelude::*;

/// The value of a pitch bend, represented as 14 bits.
///
/// A value of `0x0000` indicates full bend downwards.
/// A value of `0x2000` indicates no bend.
/// A value of `0x3FFF` indicates full bend upwards.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PitchBend<'a> {
    lsb: Cow<'a, u8>,
    msb: Cow<'a, u8>,
}

impl<'a> PitchBend<'a> {
    /// Creates a new pitch bend given the
    /// least significant and most significant bytes.
    ///
    /// Checks for byte correctness (leading 0 bit)
    pub fn new(lsb: u8, msb: u8) -> Result<Self, std::io::Error> {
        let lsb = check_u7(lsb)?;
        let msb = check_u7(msb)?;
        Ok(Self {
            lsb: Cow::Owned(lsb),
            msb: Cow::Owned(msb),
        })
    }

    /// Creates a new pitch bend given the
    /// least significant and most significant bytes.
    ///
    /// Does not check for correctness
    pub const fn new_unchecked(lsb: u8, msb: u8) -> Self {
        Self {
            lsb: Cow::Owned(lsb),
            msb: Cow::Owned(msb),
        }
    }

    /// Creates a new pitch bend given the
    /// borrowed least significant and most significant bytes.
    ///
    /// Does not check for correctness
    pub const fn new_borrowed_unchecked(lsb: &'a u8, msb: &'a u8) -> Self {
        Self {
            lsb: Cow::Borrowed(lsb),
            msb: Cow::Borrowed(msb),
        }
    }

    /// Returns a reference to the pitch bend's least significant byte.
    pub fn lsb(&self) -> &u8 {
        self.lsb.as_ref()
    }

    /// Returns a reference to the pitch bend's most significant byte.
    pub fn msb(&self) -> &u8 {
        self.msb.as_ref()
    }

    /// Represents a pitch bend, lsb then msb, as a u16
    pub fn as_bits(&self) -> u16 {
        let lsb = *self.lsb;
        let msb = *self.msb;
        let combined: u16 = ((msb as u16) << 8) | (lsb as u16);
        combined
    }

    /// Represents a u16, lsb then msb, as a pitch bend
    pub fn from_bits(rep: u16) -> Result<Self, std::io::Error> {
        let lsb = (rep >> 8) as u8;
        let msb = (rep & 0x00FF) as u8;
        Self::new(lsb, msb)
    }
}

impl PitchBend<'_> {
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
    pub fn as_int(self) -> i16 {
        self.as_bits() as i16 - 0x2000
    }

    /// Returns an `f32` in the range `[-1.0, 1.0)`.
    #[inline]
    pub fn as_f32(self) -> f32 {
        self.as_int() as f32 * (1.0 / 0x2000 as f32)
    }

    /// Returns an `f64` in the range `[-1.0, 1.0)`.
    #[inline]
    pub fn as_f64(self) -> f64 {
        self.as_int() as f64 * (1.0 / 0x2000 as f64)
    }
}
