use crate::prelude::*;

pub struct SynthesizerSettings {
    pub sample_rate: i32,
    pub block_size: usize,
    pub maximum_polyphony: usize,
    pub enable_reverb_and_chorus: bool,
}
