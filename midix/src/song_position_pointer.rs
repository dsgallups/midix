use std::io;

use crate::DataByte;

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
    lsb: DataByte,
    msb: DataByte,
}

impl SongPositionPointer {
    /// Create a new Song PositionPointer from lsb and msb bytes
    pub fn new<B, E>(lsb: B, msb: B) -> Result<Self, std::io::Error>
    where
        B: TryInto<DataByte, Error = E>,
        E: Into<io::Error>,
    {
        let lsb = lsb.try_into().map_err(Into::into)?;
        let msb = msb.try_into().map_err(Into::into)?;
        Ok(Self { lsb, msb })
    }

    /// Get the least significant byte from a SongPositionPointer
    pub fn lsb(&self) -> DataByte {
        self.lsb
    }

    /// Get the most significant byte from a SongPositionPointer
    pub fn msb(&self) -> DataByte {
        self.msb
    }
}
