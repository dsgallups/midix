use std::sync::Mutex;

use bevy::{prelude::*, tasks::IoTaskPool};
use itertools::Itertools;
use midix::prelude::ChannelVoiceMessage;
use rustysynth::{Synthesizer, SynthesizerSettings};
use tinyaudio::{OutputDeviceParameters, run_output_device};

use crate::asset::{SoundFont, SoundFontLoader};

use super::{SinkTask, Synth, SynthCommandReaderReceiver, SynthState, receiver};

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

    /// Enable reverb and chorus for the synthesizer
    pub enable_reverb_and_chorus: bool,

    /// Inserts an [`EventReader<ChannelVoiceMessage>`] that are messages send to the synth.. Disabled by default (queue will overflow if unused).
    pub synth_event_reader: bool,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            channel_count: 2,
            sample_rate: 44100,
            channel_sample_count: 441,
            enable_reverb_and_chorus: true,
            synth_event_reader: false,
        }
    }
}

/// The plugin for handling the synthesizer
#[derive(Default, Clone, Copy)]
pub struct SynthPlugin {
    /// Set params for the plugin
    pub params: SynthParams,
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

        if self.params.synth_event_reader {
            app.add_event::<ChannelVoiceMessage>().add_systems(
                PreUpdate,
                receiver::poll_receiver.run_if(in_state(SynthStatus::Loaded)),
            );
        }
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
            Self::Loaded { .. } => SynthStatus::Loaded,
        }
    }
}

impl Synth {
    fn status_should_be(&self) -> SynthStatus {
        self.synthesizer.status_should_be()
    }
}

fn load_audio_font(
    mut commands: Commands,
    mut synth: ResMut<Synth>,
    assets: Res<Assets<SoundFont>>,
) {
    let SynthState::LoadHandle { sound_font } = &synth.synthesizer else {
        warn!(
            "loading the audio font is out of sync. This is an issue with bevy_midix. Please file an issue!"
        );
        return;
    };
    let Some(sound_font) = assets.get(sound_font) else {
        return;
    };

    // the synth need not know about anything but a message to play instantaneously.
    let (synth_sender, synth_receiver) = crossbeam_channel::unbounded::<ChannelVoiceMessage>();
    let mut synth_settings = SynthesizerSettings::new(synth.params.sample_rate as i32);
    synth_settings.enable_reverb_and_chorus = synth.params.enable_reverb_and_chorus;

    let mut synthesizer = Synthesizer::new(&sound_font.file, &synth_settings).unwrap();

    let mut left = vec![0f32; synth.params.channel_sample_count];
    let mut right = vec![0f32; synth.params.channel_sample_count];

    let output_device_params = OutputDeviceParameters {
        channels_count: synth.params.channel_count,
        sample_rate: synth.params.sample_rate,
        channel_sample_count: synth.params.channel_sample_count,
    };

    let _device = if synth.params.synth_event_reader {
        let (send, recv) = crossbeam_channel::unbounded();
        commands.insert_resource(SynthCommandReaderReceiver { receiver: recv });

        run_output_device(output_device_params, {
            move |data| {
                for command in synth_receiver.try_iter() {
                    // I am uneasy about this.
                    send.try_send(command).unwrap();
                    let data1 = command.data_1_byte() as i32;
                    let data2 = command.data_2_byte().unwrap_or(0) as i32;
                    let channel = (command.status() & 0b0000_1111) as i32;
                    let command = (command.status() & 0b1111_0000) as i32;
                    synthesizer.process_midi_message(channel, command, data1, data2);
                }
                synthesizer.render(&mut left[..], &mut right[..]);
                for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                    data[i] = *value;
                }
            }
        })
    } else {
        run_output_device(output_device_params, {
            move |data| {
                for command in synth_receiver.try_iter() {
                    let data1 = command.data_1_byte() as i32;
                    let data2 = command.data_2_byte().unwrap_or(0) as i32;
                    let channel = (command.status() & 0b0000_1111) as i32;
                    let command = (command.status() & 0b1111_0000) as i32;
                    synthesizer.process_midi_message(channel, command, data1, data2);
                }
                synthesizer.render(&mut left[..], &mut right[..]);
                for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                    data[i] = *value;
                }
            }
        })
    }
    .unwrap();

    let (sink_sender, sink_receiver) = crossbeam_channel::unbounded();

    let thread_pool = IoTaskPool::get();
    thread_pool
        .spawn(SinkTask::new(synth_sender.clone(), sink_receiver))
        .detach();

    synth.synthesizer = SynthState::Loaded {
        synth_channel: synth_sender,
        sink_channel: sink_sender,
    };
    synth._device = Some(Mutex::new(_device));
}

fn state_out_of_sync(synth: Res<Synth>, current_state: Res<State<SynthStatus>>) -> bool {
    &synth.status_should_be() != current_state.get()
}

fn sync_states(synth: Res<Synth>, mut next_state: ResMut<NextState<SynthStatus>>) {
    next_state.set(synth.status_should_be());
}
