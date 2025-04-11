use crate::prelude::*;
/// Specifies a set of parameters for synthesis.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct SynthesizerSettings {
    /// The sample rate for synthesis.
    pub sample_rate: i32,
    /// The block size for rendering waveform.
    pub block_size: usize,
    /// The number of maximum polyphony.
    pub maximum_polyphony: usize,
    /// The value indicating whether reverb and chorus are enabled.
    pub enable_reverb_and_chorus: bool,
}

impl SynthesizerSettings {
    const DEFAULT_BLOCK_SIZE: usize = 64;
    const DEFAULT_MAXIMUM_POLYPHONY: usize = 64;
    const DEFAULT_ENABLE_REVERB_AND_CHORUS: bool = true;

    /// Initializes a new instance of synthesizer settings.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - The sample rate for synthesis.
    pub fn new(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            block_size: SynthesizerSettings::DEFAULT_BLOCK_SIZE,
            maximum_polyphony: SynthesizerSettings::DEFAULT_MAXIMUM_POLYPHONY,
            enable_reverb_and_chorus: SynthesizerSettings::DEFAULT_ENABLE_REVERB_AND_CHORUS,
        }
    }
}
