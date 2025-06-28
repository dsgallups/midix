#![allow(dead_code)]

use std::io;

use crate::prelude::*;

/// Reperesents the version of a SoundFont.
#[derive(Clone, Debug)]
pub struct SoundFontVersion {
    pub(crate) major: i16,
    pub(crate) minor: i16,
}

impl SoundFontVersion {
    pub(crate) fn default() -> Self {
        Self { major: 0, minor: 0 }
    }

    pub(crate) fn new<R: Read + ?Sized>(reader: &mut R) -> Result<Self, io::Error> {
        let major = BinaryReader::read_i16(reader)?;
        let minor = BinaryReader::read_i16(reader)?;

        Ok(Self { major, minor })
    }

    /// Gets the major version.
    pub fn get_major(&self) -> i32 {
        self.major as i32
    }

    /// Gets the minor version.
    pub fn get_minor(&self) -> i32 {
        self.minor as i32
    }
}
