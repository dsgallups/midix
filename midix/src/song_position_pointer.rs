use std::io;

use crate::utils::check_u7;

#[doc = r#"
 This is an internal 14 bit register that holds the number of MIDI beats (1 beat= six MIDI clocks) since the start of the song.

# Layout
This is the non-status part of a MIDI message:
`0lllllll` `0mmmmmmm`

where

`l` is the LSB, `m` the MSB.

"#]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct SongPositionPointer {
    lsb: u8,
    msb: u8,
}

impl SongPositionPointer {
    /// Create a new Song PositionPointer from lsb and msb bytes
    pub fn new(lsb: u8, msb: u8) -> Result<Self, io::Error> {
        Ok(Self {
            lsb: check_u7(lsb)?,
            msb: check_u7(msb)?,
        })
    }

    /// Create a new Song PositionPointer from lsb and msb bytes
    ///
    /// Does not check for correctness (leading 0 bit)
    pub fn new_unchecked(lsb: u8, msb: u8) -> Self {
        Self { lsb, msb }
    }

    /// Get the least significant byte from a SongPositionPointer
    pub fn lsb(&self) -> u8 {
        self.lsb
    }

    /// Get the most significant byte from a SongPositionPointer
    pub fn msb(&self) -> u8 {
        self.msb
    }
}
