use std::sync::Mutex;

use bevy::prelude::*;
use itertools::Itertools;
use midix_synth::prelude::{Synthesizer, SynthesizerSettings};
use tinyaudio::run_output_device;

use crate::asset::{SoundFont, SoundFontLoader};

use super::{Synth, SynthCommand, SynthState};

/// A lot of the docs for this struct have been copy/pasted from tiny_audio
///
/// Note that usizes are used for all params, but probably shouldn't (32 bit systems).
/// This is because the synthesizer is using tiny_audio under the hood.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SynthParams {
    /// Desired amount of audio channels. Must be at least one. Typical values: 1 - mono, 2 - stereo, etc.
    /// The data provided by the call back is _interleaved_, which means that if you have two channels then
    /// the sample layout will be like so: `LRLRLR..`, where `L` - a sample of left channel, and `R` a sample
    /// of right channel.
    pub channel_count: usize,

    /// Amount of samples per each channel. Allows you to tweak audio latency, the more the value the more
    /// latency will be and vice versa. Keep in mind, that your data callback must be able to render the
    /// samples while previous portion of data is being played, otherwise you'll get a glitchy audio.
    pub channel_sample_count: usize,

    /// Sample rate of your audio data. Typical values are: 11025 Hz, 22050 Hz, 44100 Hz (default), 48000 Hz,
    /// 96000 Hz
    pub sample_rate: usize,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            channel_count: 2,
            sample_rate: 44100,
            channel_sample_count: 441,
        }
    }
}

/// The plugin for handling the synthesizer
#[derive(Default, Clone, Copy)]
pub struct SynthPlugin {
    params: SynthParams,
}

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SoundFont>()
            .init_asset_loader::<SoundFontLoader>()
            .init_state::<SynthStatus>()
            .insert_resource(Synth::new(self.params))
            .add_systems(
                PreUpdate,
                (
                    load_audio_font.run_if(in_state(SynthStatus::ShouldLoad)),
                    sync_states.run_if(state_out_of_sync),
                )
                    .chain(),
            );
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
enum SynthStatus {
    #[default]
    NotLoaded,
    ShouldLoad,
    Loaded,
}
impl SynthState {
    fn status_should_be(&self) -> SynthStatus {
        match self {
            Self::NotLoaded => SynthStatus::NotLoaded,
            Self::LoadHandle { .. } => SynthStatus::ShouldLoad,
            Self::Loaded(..) => SynthStatus::Loaded,
        }
    }
}

impl Synth {
    fn status_should_be(&self) -> SynthStatus {
        self.synthesizer.status_should_be()
    }
}

fn load_audio_font(mut synth: ResMut<Synth>, assets: Res<Assets<SoundFont>>) {
    let SynthState::LoadHandle { sound_font } = &synth.synthesizer else {
        warn!(
            "loading the audio font is out of sync. This is an issue with bevy_midix. Please file an issue!"
        );
        return;
    };
    let Some(sound_font) = assets.get(sound_font) else {
        return;
    };

    let (sender, receiver) = crossbeam_channel::unbounded::<SynthCommand>();
    let synth_settings = SynthesizerSettings::new(synth.params.sample_rate as i32);

    let mut synthesizer = Synthesizer::new(sound_font.file.clone(), &synth_settings).unwrap();

    let mut left = vec![0f32; synth.params.channel_sample_count];
    let mut right = vec![0f32; synth.params.channel_sample_count];

    let _device = run_output_device(synth.params, {
        move |data| {
            for command in receiver.try_iter() {
                let data1 = command.event.data_1_byte();
                let data2 = command.event.data_2_byte().unwrap_or(0);
                synthesizer.process_midi_message(command.event.status(), data1, data2);
            }
            synthesizer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();
    synth.synthesizer = SynthState::Loaded(sender);
    synth._device = Some(Mutex::new(_device));
}

fn state_out_of_sync(synth: Res<Synth>, current_state: Res<State<SynthStatus>>) -> bool {
    &synth.status_should_be() != current_state.get()
}

fn sync_states(synth: Res<Synth>, mut next_state: ResMut<NextState<SynthStatus>>) {
    next_state.set(synth.status_should_be());
}
