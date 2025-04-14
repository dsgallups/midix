use crate::{prelude::*, synthesizer::RegionPair, utils};

use super::EnvelopeStage;

#[non_exhaustive]
pub struct VolumeEnvelope {
    sample_rate: i32,

    attack_slope: f64,
    decay_slope: f64,
    release_slope: f64,

    attack_start_time: f64,
    hold_start_time: f64,
    decay_start_time: f64,

    release_start_time: f64,

    sustain_level: f32,
    release_level: f32,

    processed_sample_count: usize,
    stage: EnvelopeStage,
    value: f32,
}

impl VolumeEnvelope {
    pub fn new(settings: &SynthesizerSettings, region: &RegionPair, key: u8) -> Self {
        // If the release time is shorter than 10 ms, it will be clamped to 10 ms to avoid pop noise.
        let delay = region.get_delay_volume_envelope();
        let attack = region.get_attack_volume_envelope();
        let hold = region.get_hold_volume_envelope()
            * utils::key_number_to_multiplying_factor(
                region.get_key_number_to_volume_envelope_hold(),
                key,
            );
        let decay = region.get_decay_volume_envelope()
            * utils::key_number_to_multiplying_factor(
                region.get_key_number_to_volume_envelope_decay(),
                key,
            );
        let sustain = utils::decibels_to_linear(-region.get_sustain_volume_envelope());
        let release = region.get_release_volume_envelope().max(0.01_f32);

        let attack_slope = 1_f64 / attack as f64;
        let decay_slope = -9.226_f64 / decay as f64;
        let release_slope = -9.226_f64 / release as f64;

        let attack_start_time = delay as f64;
        let hold_start_time = attack_start_time + attack as f64;
        let decay_start_time = hold_start_time + hold as f64;
        let release_start_time = 0_f64;

        // saw this twice. wtf
        let sustain_level = sustain.clamp(0., 1.);

        let release_level = 0_f32;

        let processed_sample_count = 0;
        let stage = EnvelopeStage::Delay;
        let value = 0_f32;
        let mut new = Self {
            sample_rate: settings.sample_rate,
            release_slope,
            sustain_level,
            processed_sample_count,
            stage,
            value,
            attack_slope,
            decay_slope,
            release_level,
            attack_start_time,
            hold_start_time,
            decay_start_time,
            release_start_time,
        };

        new.process(0);
        new
    }

    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.release_start_time = self.processed_sample_count as f64 / self.sample_rate as f64;
        self.release_level = self.value;
    }

    pub fn process(&mut self, sample_count: usize) -> bool {
        self.processed_sample_count += sample_count;

        let current_time = self.processed_sample_count as f64 / self.sample_rate as f64;

        // interesting...
        //
        // Maybe we can refactor this to not loop
        while self.stage <= EnvelopeStage::Hold {
            let end_time = match self.stage {
                EnvelopeStage::Delay => self.attack_start_time,
                EnvelopeStage::Attack => self.hold_start_time,
                EnvelopeStage::Hold => self.decay_start_time,
                _ => panic!("Invalid envelope stage."),
            };

            if current_time < end_time {
                break;
            } else {
                self.stage = self.stage.next();
            }
        }
        match self.stage {
            EnvelopeStage::Delay => {
                self.value = 0_f32;
                //self.priority = 4_f32 + self.value;
                true
            }
            EnvelopeStage::Attack => {
                self.value = (self.attack_slope * (current_time - self.attack_start_time)) as f32;
                //self.priority = 3_f32 + self.value;
                true
            }
            EnvelopeStage::Hold => {
                self.value = 1_f32;
                //self.priority = 2_f32 + self.value;
                true
            }
            EnvelopeStage::Decay => {
                self.value =
                    (utils::exp_cutoff(self.decay_slope * (current_time - self.decay_start_time))
                        as f32)
                        .max(self.sustain_level);

                //self.priority = 1_f32 + self.value;
                self.value > utils::NON_AUDIBLE
            }
            EnvelopeStage::Release => {
                self.value = (self.release_level as f64
                    * utils::exp_cutoff(
                        self.release_slope * (current_time - self.release_start_time),
                    )) as f32;
                //self.priority = self.value;
                self.value > utils::NON_AUDIBLE
            }
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}
