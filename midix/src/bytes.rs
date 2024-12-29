#![doc = r#"
Contains traits that are used to enforce common methods on types

# Overview
- [`AsMidiBytes`] defines something that can turn itself into a part of a MIDI file as a `Vec<u8>`. This should be removed


This module may be removed in the future.
"#]
use std::io::{self, ErrorKind};

/// A representation of some type as a part of a longer midi message
pub trait MidiBits {
    /// Identifies the bit layout (u8, u16, u32, etc.) of the type
    type BitRepresentation;

    /// Return the type as its bit representation
    fn as_bits(&self) -> Self::BitRepresentation;

    /// Attempt to create the type from the allowed [`BitRepresentation`](Self::BitRepresentation)
    ///
    /// # Errors
    /// If the bit representation cannot actually represent the type
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

/// Some data that is parsable from a midi [`ChannelVoice`](crate::prelude::ChannelVoice) message
pub trait FromMidiMessage {
    /// The minimum allowed status byte for the type
    const MIN_STATUS_BYTE: u8;

    /// The maximum allowed status byte for the type
    const MAX_STATUS_BYTE: u8;

    /// Attempt to create the type from a byte slice
    ///
    /// # Errors
    /// If the byte slice cannot actually represent the type
    fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized,
    {
        if bytes.is_empty() {
            return Err(io_error!(
                ErrorKind::InvalidInput,
                "Invalid live event (no byte data!)"
            ));
        }
        let (status, data) = bytes.split_at(1);
        let status = status[0];
        if !(Self::MIN_STATUS_BYTE..=Self::MAX_STATUS_BYTE).contains(&status) {
            return Err(io_error!(
                ErrorKind::InvalidData,
                "Invalid status message for type!)"
            ));
        }

        Self::from_status_and_data(status, data)
    }
    /// Attempt to create the type from a status and set of data.
    ///
    /// This is used mainly for comfority in [`ChannelVoice`](crate::prelude::ChannelVoice) events.
    ///
    /// # Errors
    /// If the status and data cannot represent the type
    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

/// todo
pub trait AsMidiBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

/// todo
pub trait AsMidiBytesBorrowed {
    fn borrowed_bytes(&self) -> &[u8];
}

pub(crate) trait ReadDataBytes {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error>;
}

impl ReadDataBytes for &[u8] {
    fn get_byte(&self, byte: usize) -> Result<&u8, io::Error> {
        self.get(byte).ok_or(io_error!(
            ErrorKind::InvalidInput,
            "Data not accessible for message!"
        ))
    }
}
