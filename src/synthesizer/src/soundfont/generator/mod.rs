#![allow(dead_code)]

mod r#type;
pub(crate) use r#type::*;

use crate::prelude::*;
use bevy_platform::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct Generator {
    pub(crate) generator_type: u16,
    pub(crate) value: u16,
}

impl Generator {
    fn new<R: Read + ?Sized>(reader: &mut R) -> Result<Self, SoundFontError> {
        let generator_type = BinaryReader::read_u16(reader)?;
        let value = BinaryReader::read_u16(reader)?;

        Ok(Self {
            generator_type,
            value,
        })
    }

    pub(crate) fn read_from_chunk<R: Read + ?Sized>(
        reader: &mut R,
        size: usize,
    ) -> Result<Vec<Generator>, SoundFontError> {
        if size % 4 != 0 {
            return Err(SoundFontError::InvalidGeneratorList);
        }

        let count = size / 4 - 1;

        let mut generators: Vec<Generator> = Vec::new();
        for _i in 0..count {
            generators.push(Generator::new(reader)?);
        }

        // The last one is the terminator.
        Generator::new(reader)?;

        Ok(generators)
    }
}
