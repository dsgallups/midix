use crate::{prelude::*, utils};

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
    /// we should probably combine new and start
    pub fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            sample_rate: settings.sample_rate,
            attack_slope: 0_f64,
            decay_slope: 0_f64,
            release_slope: 0_f64,
            attack_start_time: 0_f64,
            hold_start_time: 0_f64,
            decay_start_time: 0_f64,
            release_start_time: 0_f64,
            sustain_level: 0_f32,
            release_level: 0_f32,
            processed_sample_count: 0,
            stage: EnvelopeStage::Delay,
            value: 0_f32,
        }
    }

    pub fn start(
        &mut self,
        delay: f32,
        attack: f32,
        hold: f32,
        decay: f32,
        sustain: f32,
        release: f32,
    ) {
        self.attack_slope = 1_f64 / attack as f64;
        //diff 1
        self.decay_slope = -9.226_f64 / decay as f64;
        self.release_slope = -9.226_f64 / release as f64;

        self.attack_start_time = delay as f64;
        self.hold_start_time = self.attack_start_time + attack as f64;
        self.decay_start_time = self.hold_start_time + hold as f64;
        self.release_start_time = 0_f64;

        //saw this twice. wtf
        self.sustain_level = sustain.clamp(0., 1.);

        self.release_level = 0_f32;

        self.processed_sample_count = 0;
        self.stage = EnvelopeStage::Delay;
        self.value = 0_f32;

        self.process(0);
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
