mod info;

use std::vec::Vec;

pub(super) use info::ZoneInfo;

use crate::prelude::*;

pub(crate) struct Zone {
    pub(crate) generators: Vec<Generator>,
}

impl Zone {
    pub(crate) fn empty() -> Self {
        Self {
            generators: Vec::new(),
        }
    }

    fn new(info: &ZoneInfo, generators: &[Generator]) -> Self {
        let mut segment: Vec<Generator> = Vec::new();

        for i in 0..info.generator_count {
            segment.push(generators[(info.generator_index + i) as usize]);
        }

        Self {
            generators: segment,
        }
    }

    pub(crate) fn create(
        infos: &[ZoneInfo],
        generators: &[Generator],
    ) -> Result<Vec<Zone>, SoundFontError> {
        if infos.len() <= 1 {
            return Err(SoundFontError::ZoneNotFound);
        }

        // The last one is the terminator.
        let count = infos.len() - 1;

        let mut zones: Vec<Zone> = Vec::new();
        for info in infos.iter().take(count) {
            zones.push(Zone::new(info, generators));
        }

        Ok(zones)
    }
}
