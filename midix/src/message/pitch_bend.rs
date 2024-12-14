use crate::bytes::MidiBits;

/// The value of a pitch bend, represented as 14 bits.
///
/// A value of `0x0000` indicates full bend downwards.
/// A value of `0x2000` indicates no bend.
/// A value of `0x3FFF` indicates full bend upwards.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct PitchBend(u16);

impl MidiBits for PitchBend {
    type BitRepresentation = u16;
    fn as_bits(&self) -> Self::BitRepresentation {
        self.0
    }
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        todo!();
    }
}

impl PitchBend {
    /// Create a new pitch bend
    pub fn new(bend: impl Into<u16>) -> Self {
        Self(bend.into())
    }
    /// The minimum value of `0x0000`, indicating full bend downwards.
    pub const fn min_raw_value() -> PitchBend {
        PitchBend(0x0000)
    }

    /// The middle value of `0x2000`, indicating no bend.
    pub const fn mid_raw_value() -> PitchBend {
        PitchBend(0x2000)
    }

    /// The maximum value of `0x3FFF`, indicating full bend upwards.
    pub const fn max_raw_value() -> PitchBend {
        PitchBend(0x3FFF)
    }

    /// Create a `PitchBend` value from an int in the range `[-0x2000, 0x1FFF]`.
    ///
    /// Integers outside this range will be clamped.
    #[inline]
    pub fn from_int(int: i16) -> PitchBend {
        PitchBend((int.clamp(-0x2000, 0x1FFF) + 0x2000) as u16)
    }

    /// Create a `PitchBend` value from a number in the range `[-1.0, 1.0)`.
    ///
    /// Floats outside this range will be clamped.
    #[inline]
    pub fn from_f32(float: f32) -> PitchBend {
        PitchBend::from_int((float.clamp(-1.0, 1.0) * 0x2000 as f32) as i16)
    }

    /// Create a `PitchBend` value from a number in the range `[-1.0, 1.0)`.
    ///
    /// Floats outside this range will be clamped.
    #[inline]
    pub fn from_f64(float: f64) -> PitchBend {
        PitchBend::from_int((float.clamp(-1.0, 1.0) * 0x2000 as f64) as i16)
    }

    /// Returns an int in the range `[-0x2000, 0x1FFF]`.
    ///
    /// This is erroneous when writing a raw midi file. Use [`as_u16`](Self::as_u16) instead.
    #[inline]
    pub fn as_int(self) -> i16 {
        self.0 as i16 - 0x2000
    }

    /// Returns a u16. Useful when writing a midi file.
    pub fn as_u16(self) -> u16 {
        self.0
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
