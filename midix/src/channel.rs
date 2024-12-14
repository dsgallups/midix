#![doc = r#"
# Identifier for a MIDI Channel
"#]

use core::fmt;

/// Identifies a channel for MIDI, it's u4
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Channel(u8);

impl Channel {
    ///Identify a channel
    pub fn new(channel: impl Into<u8>) -> Self {
        Self(channel.into())
    }

    /// Returns the 4-bit channel number
    ///
    /// TODO: big-endian or little-endian?
    pub fn bits(&self) -> u8 {
        self.0
    }
    /// Returns the channel as a byte. top bits are zeroed.
    pub fn as_int(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
