use crate::{prelude::*, utils};

pub struct RegionPair<'a> {
    pub preset: &'a PresetRegion,
    pub instrument: &'a InstrumentRegion,
}

impl<'a> RegionPair<'a> {
    pub fn new(preset: &'a PresetRegion, instrument: &'a InstrumentRegion) -> Self {
        Self { preset, instrument }
    }

    fn gs(&self, i: usize) -> i32 {
        self.preset.gs[i] as i32 + self.instrument.gs[i] as i32
    }

    pub fn get_sample_start(&self) -> i32 {
        self.instrument.get_sample_start()
    }

    pub fn get_sample_end(&self) -> i32 {
        self.instrument.get_sample_end()
    }

    pub fn get_sample_start_loop(&self) -> i32 {
        self.instrument.get_sample_start_loop()
    }

    pub fn get_sample_end_loop(&self) -> i32 {
        self.instrument.get_sample_end_loop()
    }

    pub fn get_modulation_lfo_to_pitch(&self) -> i32 {
        self.gs(GeneratorType::MODULATION_LFO_TO_PITCH as usize)
    }

    pub fn get_vibrato_lfo_to_pitch(&self) -> i32 {
        self.gs(GeneratorType::VIBRATO_LFO_TO_PITCH as usize)
    }

    pub fn get_modulation_envelope_to_pitch(&self) -> i32 {
        self.gs(GeneratorType::MODULATION_ENVELOPE_TO_PITCH as usize)
    }

    pub fn get_initial_filter_cutoff_frequency(&self) -> f32 {
        utils::cents_to_hertz(
            self.gs(GeneratorType::INITIAL_FILTER_CUTOFF_FREQUENCY as usize) as f32,
        )
    }

    pub fn get_initial_filter_q(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::INITIAL_FILTER_Q as usize) as f32
    }

    pub fn get_modulation_lfo_to_filter_cutoff_frequency(&self) -> i32 {
        self.gs(GeneratorType::MODULATION_LFO_TO_FILTER_CUTOFF_FREQUENCY as usize)
    }

    pub fn get_modulation_envelope_to_filter_cutoff_frequency(&self) -> i32 {
        self.gs(GeneratorType::MODULATION_ENVELOPE_TO_FILTER_CUTOFF_FREQUENCY as usize)
    }

    pub fn get_modulation_lfo_to_volume(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::MODULATION_LFO_TO_VOLUME as usize) as f32
    }

    pub fn get_chorus_effects_send(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::CHORUS_EFFECTS_SEND as usize) as f32
    }

    pub fn get_reverb_effects_send(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::REVERB_EFFECTS_SEND as usize) as f32
    }

    pub fn get_pan(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::PAN as usize) as f32
    }

    pub fn get_delay_modulation_lfo(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::DELAY_MODULATION_LFO as usize) as f32)
    }

    pub fn get_frequency_modulation_lfo(&self) -> f32 {
        utils::cents_to_hertz(self.gs(GeneratorType::FREQUENCY_MODULATION_LFO as usize) as f32)
    }

    pub fn get_delay_vibrato_lfo(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::DELAY_VIBRATO_LFO as usize) as f32)
    }

    pub fn get_frequency_vibrato_lfo(&self) -> f32 {
        utils::cents_to_hertz(self.gs(GeneratorType::FREQUENCY_VIBRATO_LFO as usize) as f32)
    }

    pub fn get_delay_modulation_envelope(&self) -> f32 {
        utils::timecents_to_seconds(
            self.gs(GeneratorType::DELAY_MODULATION_ENVELOPE as usize) as f32
        )
    }

    pub fn get_attack_modulation_envelope(&self) -> f32 {
        utils::timecents_to_seconds(
            self.gs(GeneratorType::ATTACK_MODULATION_ENVELOPE as usize) as f32
        )
    }

    pub fn get_hold_modulation_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::HOLD_MODULATION_ENVELOPE as usize) as f32)
    }

    pub fn get_decay_modulation_envelope(&self) -> f32 {
        utils::timecents_to_seconds(
            self.gs(GeneratorType::DECAY_MODULATION_ENVELOPE as usize) as f32
        )
    }

    pub fn get_sustain_modulation_envelope(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::SUSTAIN_MODULATION_ENVELOPE as usize) as f32
    }

    pub fn get_release_modulation_envelope(&self) -> f32 {
        utils::timecents_to_seconds(
            self.gs(GeneratorType::RELEASE_MODULATION_ENVELOPE as usize) as f32
        )
    }

    pub fn get_key_number_to_modulation_envelope_hold(&self) -> i32 {
        self.gs(GeneratorType::KEY_NUMBER_TO_MODULATION_ENVELOPE_HOLD as usize)
    }

    pub fn get_key_number_to_modulation_envelope_decay(&self) -> i32 {
        self.gs(GeneratorType::KEY_NUMBER_TO_MODULATION_ENVELOPE_DECAY as usize)
    }

    pub fn get_delay_volume_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::DELAY_VOLUME_ENVELOPE as usize) as f32)
    }

    pub fn get_attack_volume_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::ATTACK_VOLUME_ENVELOPE as usize) as f32)
    }

    pub fn get_hold_volume_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::HOLD_VOLUME_ENVELOPE as usize) as f32)
    }

    pub fn get_decay_volume_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::DECAY_VOLUME_ENVELOPE as usize) as f32)
    }

    pub fn get_sustain_volume_envelope(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::SUSTAIN_VOLUME_ENVELOPE as usize) as f32
    }

    pub fn get_release_volume_envelope(&self) -> f32 {
        utils::timecents_to_seconds(self.gs(GeneratorType::RELEASE_VOLUME_ENVELOPE as usize) as f32)
    }

    pub fn get_key_number_to_volume_envelope_hold(&self) -> i32 {
        self.gs(GeneratorType::KEY_NUMBER_TO_VOLUME_ENVELOPE_HOLD as usize)
    }

    pub fn get_key_number_to_volume_envelope_decay(&self) -> i32 {
        self.gs(GeneratorType::KEY_NUMBER_TO_VOLUME_ENVELOPE_DECAY as usize)
    }

    pub fn get_initial_attenuation(&self) -> f32 {
        0.1_f32 * self.gs(GeneratorType::INITIAL_ATTENUATION as usize) as f32
    }

    pub fn get_coarse_tune(&self) -> i32 {
        self.gs(GeneratorType::COARSE_TUNE as usize)
    }

    pub fn get_fine_tune(&self) -> i32 {
        self.gs(GeneratorType::FINE_TUNE as usize) + self.instrument.sample_pitch_correction
    }

    pub fn get_sample_modes(&self) -> LoopMode {
        self.instrument.get_sample_modes()
    }

    pub fn get_scale_tuning(&self) -> i32 {
        self.gs(GeneratorType::SCALE_TUNING as usize)
    }

    pub fn get_exclusive_class(&self) -> i32 {
        self.instrument.get_exclusive_class()
    }

    pub fn get_root_key(&self) -> i32 {
        self.instrument.get_root_key()
    }
}
