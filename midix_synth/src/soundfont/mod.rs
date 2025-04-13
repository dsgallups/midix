#![allow(dead_code)]

use std::io::Read;
use std::sync::Arc;
use tracing::warn;

pub mod generator;
pub mod instrument;
pub mod preset;
pub mod zone;

mod info;
pub use info::*;
pub(super) mod math;
mod parameters;
mod sampledata;
use parameters::SoundFontParameters;
pub use sampledata::*;
mod version;
pub use version::*;
mod error;
pub use error::*;
mod sample_header;
pub use sample_header::*;

use crate::prelude::*;

/// Reperesents a SoundFont.
#[non_exhaustive]
pub struct SoundFont {
    pub(crate) info: SoundFontInfo,
    pub(crate) bits_per_sample: i32,
    pub(crate) wave_data: Arc<Vec<i16>>,
    pub(crate) sample_headers: Vec<SampleHeader>,
    pub(crate) presets: Vec<Preset>,
    pub(crate) instruments: Vec<Instrument>,
}

impl SoundFont {
    /// Loads a SoundFont from the stream.
    ///
    /// # Arguments
    ///
    /// * `reader` - The data stream used to load the SoundFont.
    pub fn new<R: Read + ?Sized>(reader: &mut R) -> Result<Self, SoundFontError> {
        let chunk_id = BinaryReader::read_four_cc(reader)?;
        if chunk_id != b"RIFF" {
            return Err(SoundFontError::RiffChunkNotFound);
        }

        let _size = BinaryReader::read_i32(reader);

        let form_type = BinaryReader::read_four_cc(reader)?;
        if form_type != b"sfbk" {
            return Err(SoundFontError::InvalidRiffChunkType {
                expected: FourCC::from_bytes(*b"sfbk"),
                actual: form_type,
            });
        }

        let info = SoundFontInfo::new(reader)?;
        let sample_data = SoundFontSampleData::new(reader)?;
        let parameters = SoundFontParameters::new(reader)?;

        let sound_font = Self {
            info,
            bits_per_sample: 16,
            wave_data: Arc::new(sample_data.wave_data),
            sample_headers: parameters.sample_headers,
            presets: parameters.presets,
            instruments: parameters.instruments,
        };

        sound_font.sanity_check()?;

        Ok(sound_font)
    }

    fn sanity_check(&self) -> Result<(), SoundFontError> {
        // https://github.com/sinshu/rustysynth/issues/22
        // https://github.com/sinshu/rustysynth/issues/33
        for instrument in &self.instruments {
            for region in &instrument.regions {
                let start = region.get_sample_start();
                let end = region.get_sample_end();
                let start_loop = region.get_sample_start_loop();
                let end_loop = region.get_sample_end_loop();

                if start < 0
                    || start_loop < 0
                    || end as usize >= self.wave_data.len()
                    || end_loop as usize >= self.wave_data.len()
                    || end < start
                    || end_loop < start_loop
                {
                    warn!(
                        "Something is wrong with this soundfont. The following conditions should all be false:\n\
                        start < 0: {}\n\
                        start_loop < 0: {}\n\
                        end >= self.wave_data.len(): {}\n\
                        end_loop >= self.wave_data.len() {}\n\
                        end < start: {}\n\
                        end_loop < start_loop: {}\n\
                        \n\
                        Variables:\n\
                        start: {start}\n\
                        end: {end}\n\
                        start_loop:{start_loop}\n\
                        end_loop:{end_loop}\n\
                        wave_data.len(): {}\n",
                        start < 0,
                        start_loop < 0,
                        end as usize >= self.wave_data.len(),
                        end_loop as usize >= self.wave_data.len(),
                        end < start,
                        end_loop < start_loop,
                        self.wave_data.len(),
                    );
                    //return Err(SoundFontError::SanityCheckFailed);
                }
            }
        }

        Ok(())
    }

    /// Gets the information of the SoundFont.
    pub fn get_info(&self) -> &SoundFontInfo {
        &self.info
    }

    /// Gets the bits per sample of the sample data.
    pub fn get_bits_per_sample(&self) -> i32 {
        self.bits_per_sample
    }

    /// Gets the sample data.
    pub fn get_wave_data(&self) -> &[i16] {
        &self.wave_data[..]
    }

    /// Gets the samples of the SoundFont.
    pub fn get_sample_headers(&self) -> &[SampleHeader] {
        &self.sample_headers[..]
    }

    /// Gets the presets of the SoundFont.
    pub fn get_presets(&self) -> &[Preset] {
        &self.presets[..]
    }

    /// Gets the instruments of the SoundFont.
    pub fn get_instruments(&self) -> &[Instrument] {
        &self.instruments[..]
    }
}
