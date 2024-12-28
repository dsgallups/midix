#![doc = r#"
# Identifier for a MIDI Channel
"#]

use core::fmt;
use std::borrow::Cow;

use crate::utils::check_u4;

/// Identifies a channel for MIDI, it's u4
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Channel<'a>(Cow<'a, u8>);

impl<'a> Channel<'a> {
    ///Identify a channel
    pub fn new(channel: u8) -> Result<Self, std::io::Error> {
        Ok(Self(Cow::Owned(check_u4(channel)?)))
    }

    pub fn from_status(status: u8) -> Self {
        let channel = status & 0b0000_1111;
        println!("status, channel: {}, {}", status, channel);
        Self(Cow::Owned(channel + 1))
    }

    /// Returns the 4-bit channel number
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl fmt::Display for Channel<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
