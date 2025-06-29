use crate::prelude::{voice::RegionPair, *};

use super::EnvelopeStage;

pub struct ModulationEnvelope {
    sample_rate: i32,

    attack_slope: f64,
    decay_slope: f64,
    release_slope: f64,

    attack_start_time: f64,
    hold_start_time: f64,
    decay_start_time: f64,

    decay_end_time: f64,
    release_end_time: f64,

    sustain_level: f32,
    release_level: f32,

    processed_sample_count: usize,
    stage: EnvelopeStage,
    value: f32,
}

impl ModulationEnvelope {
    pub fn new(settings: &SynthesizerSettings, region: &RegionPair, key: u8, velocity: u8) -> Self {
        // According to the implementation of TinySoundFont, the attack time should be adjusted by the velocity.
        let delay = region.get_delay_modulation_envelope();
        let attack = region.get_attack_modulation_envelope() * ((145 - velocity) as f32 / 144_f32);
        let hold = region.get_hold_modulation_envelope()
            * utils::key_number_to_multiplying_factor(
                region.get_key_number_to_modulation_envelope_hold(),
                key,
            );
        let decay = region.get_decay_modulation_envelope()
            * utils::key_number_to_multiplying_factor(
                region.get_key_number_to_modulation_envelope_decay(),
                key,
            );
        let sustain = 1_f32 - region.get_sustain_modulation_envelope() / 100_f32;
        let release = region.get_release_modulation_envelope();
        let attack_slope = 1_f64 / attack as f64;
        let decay_slope = 1_f64 / decay as f64;
        let release_slope = 1_f64 / release as f64;

        let attack_start_time = delay as f64;
        let hold_start_time = attack_start_time + attack as f64;
        let decay_start_time = hold_start_time + hold as f64;

        let decay_end_time = decay_start_time + decay as f64;
        let release_end_time = release as f64;

        let sustain_level = sustain.clamp(0., 1.);

        let release_level = 0_f32;

        let processed_sample_count = 0;
        let stage = EnvelopeStage::Delay;
        let value = 0_f32;
        let mut new = Self {
            sample_rate: settings.sample_rate,
            attack_slope,
            decay_slope,
            release_slope,
            attack_start_time,
            hold_start_time,
            decay_start_time,
            decay_end_time,
            release_end_time,
            sustain_level,
            release_level,
            processed_sample_count,
            stage,
            value,
        };

        new.process(0);
        new
    }

    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
        self.release_end_time += self.processed_sample_count as f64 / self.sample_rate as f64;
        self.release_level = self.value;
    }

    pub fn process(&mut self, sample_count: usize) -> Option<f32> {
        self.processed_sample_count += sample_count;

        let current_time = self.processed_sample_count as f64 / self.sample_rate as f64;

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
                self.value = 0.;
                Some(0.)
            }
            EnvelopeStage::Attack => {
                self.value = (self.attack_slope * (current_time - self.attack_start_time)) as f32;
                Some(self.value)
            }
            EnvelopeStage::Hold => {
                self.value = 1.;
                Some(1.)
            }
            EnvelopeStage::Decay => {
                let val = ((self.decay_slope * (self.decay_end_time - current_time)) as f32)
                    .max(self.sustain_level);
                self.value = val;
                (val > utils::NON_AUDIBLE).then_some(val)
            }
            EnvelopeStage::Release => {
                let val = ((self.release_level as f64
                    * self.release_slope
                    * (self.release_end_time - current_time)) as f32)
                    .max(0.);
                self.value = val;

                (val > utils::NON_AUDIBLE).then_some(val)
            }
        }
    }
}
