use std::f32::consts;

mod envelope;
use envelope::*;
mod region;
pub(super) use region::*;

mod oscillator;
use oscillator::*;

mod bi_quad_filter;
use bi_quad_filter::*;

use crate::{prelude::*, utils};

use super::Channel;

pub(crate) struct Voice {
    block_size: usize,

    vol_env: VolumeEnvelope,
    mod_env: ModulationEnvelope,

    vib_lfo: Lfo,
    mod_lfo: Lfo,

    oscillator: Oscillator,
    filter: BiQuadFilter,

    pub(crate) block: Vec<f32>,

    // A sudden change in the mix gain will cause pop noise.
    // To avoid this, we save the mix gain of the previous block,
    // and smooth out the gain if the gap between the current and previous gain is too large.
    // The actual smoothing process is done in the WriteBlock method of the Synthesizer class.
    pub(crate) previous_mix_gain_left: f32,
    pub(crate) previous_mix_gain_right: f32,
    pub(crate) current_mix_gain_left: f32,
    pub(crate) current_mix_gain_right: f32,

    pub(crate) previous_reverb_send: f32,
    pub(crate) previous_chorus_send: f32,
    pub(crate) current_reverb_send: f32,
    pub(crate) current_chorus_send: f32,

    pub(crate) exclusive_class: i32,
    pub(crate) channel: u8,
    pub(crate) key: u8,

    note_gain: f32,

    cutoff: f32,
    resonance: f32,

    vib_lfo_to_pitch: f32,
    mod_lfo_to_pitch: f32,
    mod_env_to_pitch: f32,

    mod_lfo_to_cutoff: i32,
    mod_env_to_cutoff: i32,
    dynamic_cutoff: bool,

    mod_lfo_to_volume: f32,
    dynamic_volume: bool,

    instrument_pan: f32,
    instrument_reverb: f32,
    instrument_chorus: f32,

    // Some instruments require fast cutoff change, which can cause pop noise.
    // This is used to smooth out the cutoff frequency.
    smoothed_cutoff: f32,

    voice_state: i32,
    pub(crate) voice_length: usize,
    min_voice_length: usize,
}

impl Voice {
    pub(crate) fn new(
        settings: &SynthesizerSettings,
        region: &RegionPair,
        channel: u8,
        key: u8,
        velocity: u8,
    ) -> Self {
        // this is used elsewhere...really thinking we should
        // just use the region
        let exclusive_class = region.get_exclusive_class();

        let note_gain = if velocity > 0 {
            // According to the Polyphone's implementation, the initial attenuation should be reduced to 40%.
            // I'm not sure why, but this indeed improves the loudness variability.
            let sample_attenuation = 0.4_f32 * region.get_initial_attenuation();
            let filter_attenuation = 0.5_f32 * region.get_initial_filter_q();
            let decibels = 2_f32 * utils::linear_to_decibels(velocity as f32 / 127_f32)
                - sample_attenuation
                - filter_attenuation;
            utils::decibels_to_linear(decibels)
        } else {
            0_f32
        };

        let cutoff = region.get_initial_filter_cutoff_frequency();
        let resonance = utils::decibels_to_linear(region.get_initial_filter_q());

        let vib_lfo_to_pitch = 0.01_f32 * region.get_vibrato_lfo_to_pitch() as f32;
        let mod_lfo_to_pitch = 0.01_f32 * region.get_modulation_lfo_to_pitch() as f32;
        let mod_env_to_pitch = 0.01_f32 * region.get_modulation_envelope_to_pitch() as f32;

        let mod_lfo_to_cutoff = region.get_modulation_lfo_to_filter_cutoff_frequency();
        let mod_env_to_cutoff = region.get_modulation_envelope_to_filter_cutoff_frequency();
        //todo: derivable and cheap.
        let dynamic_cutoff = mod_lfo_to_cutoff != 0 || mod_env_to_cutoff != 0;

        let mod_lfo_to_volume = region.get_modulation_lfo_to_volume();
        let dynamic_volume = mod_lfo_to_volume > 0.05_f32;

        let instrument_pan = region.get_pan().clamp(-50., 50.);

        let instrument_reverb = 0.01_f32 * region.get_reverb_effects_send();
        let instrument_chorus = 0.01_f32 * region.get_chorus_effects_send();

        let vol_env = VolumeEnvelope::new(settings, region, key);
        let mod_env = ModulationEnvelope::new(settings, region, key, velocity);

        let vib_lfo = Lfo::new(
            settings,
            region.get_delay_vibrato_lfo(),
            region.get_frequency_vibrato_lfo(),
        );
        let mod_lfo = Lfo::new(
            settings,
            region.get_delay_modulation_lfo(),
            region.get_frequency_modulation_lfo(),
        );

        let oscillator = Oscillator::new(settings, region);

        let mut filter = BiQuadFilter::new(settings);
        filter.clear_buffer();
        filter.set_low_pass_filter(cutoff, resonance);

        let smoothed_cutoff = cutoff;

        let voice_state = VoiceState::PLAYING;
        //???
        let voice_length = 0;

        //???
        let min_voice_length = (settings.sample_rate / 500) as usize;
        Self {
            block_size: settings.block_size,
            vol_env,
            mod_env,
            vib_lfo,
            mod_lfo,
            oscillator,
            filter,
            block: vec![0_f32; settings.block_size],
            previous_mix_gain_left: 0_f32,
            previous_mix_gain_right: 0_f32,
            current_mix_gain_left: 0_f32,
            current_mix_gain_right: 0_f32,
            previous_reverb_send: 0_f32,
            previous_chorus_send: 0_f32,
            current_reverb_send: 0_f32,
            current_chorus_send: 0_f32,
            exclusive_class,
            channel,
            key,
            note_gain,
            cutoff,
            resonance,
            vib_lfo_to_pitch,
            mod_lfo_to_pitch,
            mod_env_to_pitch,
            mod_lfo_to_cutoff,
            mod_env_to_cutoff,
            dynamic_cutoff,
            mod_lfo_to_volume,
            dynamic_volume,
            instrument_pan,
            instrument_reverb,
            instrument_chorus,
            smoothed_cutoff,
            voice_state,
            voice_length,
            min_voice_length,
        }
    }

    pub(crate) fn end(&mut self) {
        if self.voice_state == VoiceState::PLAYING {
            self.voice_state = VoiceState::RELEASE_REQUESTED;
        }
    }

    // /// Note stops immediately without a release sound.
    // ///
    // /// End is *supposed* to begin playing a release sound. this is the
    // /// evil twin.
    // ///
    // /// This also means it will drop on the next process call.
    // pub(crate) fn kill(&mut self) {
    //     self.note_gain = 0_f32;
    // }

    /// this is only called in one place: render_block. If I return false,
    /// I will die.
    ///
    /// When do I die?
    ///
    /// 1. if my note_gain is less than NON_audible
    /// 2. if my volume envelope determines I am no longer audible
    /// 3. mod env is just hanging around, so it's definitely not supposed to
    ///    return a bool
    ///
    pub(crate) fn process(&mut self, data: &[i16], channels: &[Channel]) -> bool {
        if self.note_gain < utils::NON_AUDIBLE {
            return false;
        }

        let channel_info = &channels[self.channel as usize];

        self.release_if_necessary(channel_info);

        let Some(vol_env) = self.vol_env.process(self.block_size) else {
            return false;
        };

        let Some(mod_env) = self.mod_env.process(self.block_size) else {
            return false;
        };
        let vib_lfo = self.vib_lfo.process();
        let mod_lfo = self.mod_lfo.process();

        let vib_pitch_change =
            (0.01_f32 * channel_info.get_modulation() + self.vib_lfo_to_pitch) * vib_lfo;
        let mod_pitch_change = self.mod_lfo_to_pitch * mod_lfo + self.mod_env_to_pitch * mod_env;
        let channel_pitch_change = channel_info.get_tune() + channel_info.get_pitch_bend();
        let pitch = self.key as f32 + vib_pitch_change + mod_pitch_change + channel_pitch_change;
        if !self.oscillator.process(data, &mut self.block[..], pitch) {
            return false;
        }

        if self.dynamic_cutoff {
            let cents =
                self.mod_lfo_to_cutoff as f32 * mod_lfo + self.mod_env_to_cutoff as f32 * mod_env;
            let factor = utils::cents_to_multiplying_factor(cents);
            let new_cutoff = factor * self.cutoff;

            // The cutoff change is limited within x0.5 and x2 to reduce pop noise.
            let lower_limit = 0.5_f32 * self.smoothed_cutoff;
            let upper_limit = 2_f32 * self.smoothed_cutoff;

            self.smoothed_cutoff = new_cutoff.clamp(lower_limit, upper_limit);

            self.filter
                .set_low_pass_filter(self.smoothed_cutoff, self.resonance);
        }
        self.filter.process(&mut self.block[..]);

        self.previous_mix_gain_left = self.current_mix_gain_left;
        self.previous_mix_gain_right = self.current_mix_gain_right;
        self.previous_reverb_send = self.current_reverb_send;
        self.previous_chorus_send = self.current_chorus_send;

        // According to the GM spec, the following value should be squared.
        let ve = channel_info.get_volume() * channel_info.get_expression();
        let channel_gain = ve * ve;

        let mut mix_gain = self.note_gain * channel_gain * vol_env;
        if self.dynamic_volume {
            let decibels = self.mod_lfo_to_volume * mod_lfo;
            mix_gain *= utils::decibels_to_linear(decibels);
        }

        let angle =
            (consts::PI / 200_f32) * (channel_info.get_pan() + self.instrument_pan + 50_f32);
        if angle <= 0_f32 {
            self.current_mix_gain_left = mix_gain;
            self.current_mix_gain_right = 0_f32;
        } else if angle >= utils::HALF_PI {
            self.current_mix_gain_left = 0_f32;
            self.current_mix_gain_right = mix_gain;
        } else {
            self.current_mix_gain_left = mix_gain * angle.cos();
            self.current_mix_gain_right = mix_gain * angle.sin();
        }

        self.current_reverb_send =
            (channel_info.get_reverb_send() + self.instrument_reverb).clamp(0., 1.);

        self.current_chorus_send =
            (channel_info.get_chorus_send() + self.instrument_chorus).clamp(0., 1.);

        if self.voice_length == 0 {
            self.previous_mix_gain_left = self.current_mix_gain_left;
            self.previous_mix_gain_right = self.current_mix_gain_right;
            self.previous_reverb_send = self.current_reverb_send;
            self.previous_chorus_send = self.current_chorus_send;
        }

        self.voice_length += self.block_size;

        true
    }

    fn release_if_necessary(&mut self, channel_info: &Channel) {
        if self.voice_length < self.min_voice_length {
            return;
        }

        if self.voice_state == VoiceState::RELEASE_REQUESTED && !channel_info.get_hold_pedal() {
            self.vol_env.release();
            self.mod_env.release();
            self.oscillator.release();

            self.voice_state = VoiceState::RELEASED;
        }
    }
}

#[allow(unused)]
#[non_exhaustive]
struct VoiceState {}

impl VoiceState {
    const PLAYING: i32 = 0;
    const RELEASE_REQUESTED: i32 = 1;
    const RELEASED: i32 = 2;
}
