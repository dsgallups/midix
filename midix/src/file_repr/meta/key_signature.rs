use core::ops::Neg;

#[doc = r#"
Defines the key signature of a MIDI file.

# Layout
FF 59 02 sf mi Key Signature
sf = -7: 7 flats
sf = -1: 1 flat
sf = 0: key of C
sf = 1: 1 sharp
sf = 7: 7 sharps

mi = 0: major key
mi = 1: minor key
"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct KeySignature([u8; 2]);
impl KeySignature {
    /// Create a new key signature from a byte slice
    pub const fn new_from_bytes(v: [u8; 2]) -> Self {
        Self(v)
    }

    /// Count the number of flats or sharps. a positive number
    /// indicates a number of sharps. a negative number indicates
    /// a number of flats.
    pub const fn sharp_flat_count(&self) -> i8 {
        self.0[0] as i8
    }

    /// the identifiable count of sharps
    pub fn num_sharps(&self) -> u8 {
        self.sharp_flat_count().min(0).unsigned_abs()
    }

    /// the identifiable count of flats
    pub fn num_flats(&self) -> u8 {
        self.sharp_flat_count().neg().min(0).unsigned_abs()
    }
    /// True if the key is identified as minor
    pub const fn minor_key(&self) -> bool {
        self.0[1] == 1
    }
}
