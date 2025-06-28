use crate::{bevy::synth::SynthState, prelude::ChannelVoiceMessage};
use bevy::prelude::*;
use bevy_platform::time::Instant;
use crossbeam_channel::{Receiver, Sender, TryIter};

#[cfg(feature = "std")]
use bevy_platform::sync::Mutex;

#[cfg(feature = "std")]
use itertools::Itertools;
#[cfg(feature = "std")]
use rustysynth::{Synthesizer, SynthesizerSettings};
#[cfg(feature = "std")]
use tinyaudio::{OutputDeviceParameters, run_output_device};

use crate::bevy::prelude::*;

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
    pub synth_event_reader: SynthEventOpt,
}

/// Options for reading ALL events sent to the synth
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynthEventOpt {
    /// Do not insert an event reader
    None,
    /// Insert a [`SynthEventReceiver`], but do not insert an event writer.
    ///
    /// e.g. "I will handle the events myself for a single thing that needs to know"
    ReceiverOnly,
    /// Insert an [`EventWriter<SynthEvent>`] that will propagate events.
    ///
    /// Note: if you also use [`SynthEventReceiver`] in your systems, be prepared to miss
    /// events from this writer! (the receiver does not clone a shared buffer)
    EventWriter,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            channel_count: 2,
            sample_rate: 44100,
            channel_sample_count: 441,
            enable_reverb_and_chorus: true,
            synth_event_reader: SynthEventOpt::None,
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
        #[cfg(feature = "std")]
        app.init_asset::<SoundFontAsset>()
            .init_asset_loader::<SoundFontLoader>();
        app.init_state::<SynthStatus>()
            .insert_resource(Synth::new(self.params))
            .add_systems(
                PreUpdate,
                (
                    #[cfg(feature = "std")]
                    load_audio_font.run_if(in_state(SynthStatus::ShouldLoad)),
                    sync_states.run_if(state_out_of_sync),
                )
                    .chain(),
            );

        match self.params.synth_event_reader {
            SynthEventOpt::EventWriter => {
                app.init_resource::<SynthEventReceiver>()
                    .add_event::<SynthEvent>()
                    .add_systems(
                        PreUpdate,
                        poll_receiver.run_if(in_state(SynthStatus::Loaded)),
                    );
            }
            SynthEventOpt::ReceiverOnly => {
                app.init_resource::<SynthEventReceiver>();
            }
            _ => {}
        };
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
enum SynthStatus {
    #[default]
    NotLoaded,
    #[cfg(feature = "std")]
    ShouldLoad,
    Loaded,
}
impl SynthState {
    fn status_should_be(&self) -> SynthStatus {
        match self {
            Self::NotLoaded => SynthStatus::NotLoaded,
            #[cfg(feature = "std")]
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

#[cfg(feature = "std")]
fn load_audio_font(
    receiver: Option<ResMut<SynthEventReceiver>>,
    mut synth: ResMut<Synth>,
    assets: Res<Assets<SoundFontAsset>>,
) {
    use bevy::tasks::IoTaskPool;

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

    let _device = if let Some(mut receiver) = receiver {
        let send = receiver.take_sender();
        run_output_device(output_device_params, {
            move |data| {
                for command in synth_receiver.try_iter() {
                    // I am uneasy about this.
                    send.try_send(SynthEvent {
                        received: Instant::now(),
                        message: command,
                    })
                    .unwrap();
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

/// Wraps a message and timestamp upon the synth
/// receiving a message
#[derive(Event)]
pub struct SynthEvent {
    /// The instant the synth thread received the message
    pub received: Instant,
    /// The message in question
    pub message: ChannelVoiceMessage,
}

/// This is a reader that will recieve commands sent to the synth.
///
/// You can propagate the receiver inner to an `EventWriter<SynthEvent>` if you
/// have multiple systems that need to deal with events. Otherwise, you can use
/// this directly and poll all the events
#[derive(Resource, Component, Clone)]
pub struct SynthEventReceiver {
    // holds onto this until the synth thread spawns
    _sender: Option<Sender<SynthEvent>>,
    receiver: Receiver<SynthEvent>,
}

impl Default for SynthEventReceiver {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self {
            _sender: Some(sender),
            receiver,
        }
    }
}

impl SynthEventReceiver {
    /// Non blocking iterator of events
    #[inline]
    pub fn try_iter(&self) -> TryIter<'_, SynthEvent> {
        self.receiver.try_iter()
    }

    /// Blocking iterator of events
    #[inline]
    pub fn iter(&self) -> crossbeam_channel::Iter<'_, SynthEvent> {
        self.receiver.iter()
    }
    /// Get a reference to the underlying receiver
    pub fn receiver(&self) -> &Receiver<SynthEvent> {
        &self.receiver
    }
    #[allow(dead_code)]
    pub(crate) fn take_sender(&mut self) -> Sender<SynthEvent> {
        self._sender.take().unwrap()
    }
}

/// connects the channel with the resource
fn poll_receiver(mut ev: EventWriter<SynthEvent>, command_receiver: Res<SynthEventReceiver>) {
    ev.write_batch(command_receiver.receiver.try_iter());
}
