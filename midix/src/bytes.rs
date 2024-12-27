//! Contains traits that can represent structs
//!
use std::io::{self, ErrorKind};

/// A representation of some type as a part of a longer midi message
pub trait MidiBits {
    type BitRepresentation;
    fn as_bits(&self) -> Self::BitRepresentation;
    fn from_bits(rep: Self::BitRepresentation) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

/// Some data that is parsable from a midi message
pub trait FromMidiMessage {
    const MIN_STATUS_BYTE: u8;
    const MAX_STATUS_BYTE: u8;
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

    fn from_status_and_data(status: u8, data: &[u8]) -> Result<Self, std::io::Error>
    where
        Self: Sized;
}

pub trait AsMidiBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

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
