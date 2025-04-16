#![allow(dead_code)]
use crate::prelude::*;

use super::EnvelopeStage;

// I think we can process volume and modulation
// at the same time.
pub struct CommonEnvelope {
    pub sample_rate: i32,

    pub attack_slope: f64,
    pub decay_slope: f64,
    pub release_slope: f64,

    pub attack_start_time: f64,
    pub hold_start_time: f64,
    pub decay_start_time: f64,

    pub sustain_level: f32,
    pub release_level: f32,

    pub processed_sample_count: usize,
    pub stage: EnvelopeStage,
    pub value: f32,
}

impl CommonEnvelope {
    pub fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            sample_rate: settings.sample_rate,
            attack_slope: 0_f64,
            decay_slope: 0_f64,
            release_slope: 0_f64,
            attack_start_time: 0_f64,
            hold_start_time: 0_f64,
            decay_start_time: 0_f64,
            sustain_level: 0_f32,
            release_level: 0_f32,
            processed_sample_count: 0,
            stage: EnvelopeStage::Delay,
            value: 0_f32,
        }
    }

    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.release_level = self.value;
    }
}
