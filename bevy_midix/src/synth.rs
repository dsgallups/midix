#![doc = r#"
Synthesizer resources, setup and plugins
"#]

use crate::prelude::SoundFont;
use crate::prelude::*;
use bevy::prelude::*;
use itertools::Itertools;
use midix_synth::prelude::*;
use std::sync::{Arc, Mutex};
use tinyaudio::{run_output_device, OutputDevice, OutputDeviceParameters};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum SynthStatus {
    NotLoaded,
    ShouldLoad,
    Loaded,
}

/// The plugin for handling the synthesizer
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_if_not_inserted).add_systems(
        PreUpdate,
        (
            load_audio_font.run_if(in_state(SynthStatus::ShouldLoad)),
            sync_states.run_if(state_out_of_sync),
        )
            .chain(),
    );
}

enum SynthState {
    NotLoaded,
    LoadHandle { sound_font: Handle<SoundFont> },
    Loaded(Arc<Mutex<Synthesizer>>),
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

/// Plays audio commands with the provided soundfont
///
/// Pass the synth midi events via the `Synth::handle_event` method
///
/// see [`ChannelVoiceMessage`] for the list of options
#[derive(Resource)]
pub struct Synth {
    params: OutputDeviceParameters,
    synthesizer: SynthState,
    _device: Option<Mutex<OutputDevice>>,
}

impl Synth {
    /// Create a new synth given the following parameters:
    ///
    /// 1. The number of output channels
    ///
    /// A good default is 2? I actually don't know
    ///
    /// 2. The sampling rate for the audio font (if this needs more info, please file an issue for docs)
    ///
    /// A good default is 44100
    ///
    /// 3. The sample count for each channel
    ///
    /// A good default is 441
    pub fn new(channel_count: usize, sample_rate: usize, channel_sample_count: usize) -> Self {
        Self {
            params: OutputDeviceParameters {
                channels_count: channel_count,
                sample_rate,
                channel_sample_count,
            },
            ..Default::default()
        }
    }
    fn status_should_be(&self) -> SynthStatus {
        self.synthesizer.status_should_be()
    }
    /// Send an event for the synth to play
    pub fn handle_event(&self, event: ChannelVoiceMessage) {
        let SynthState::Loaded(synth) = &self.synthesizer else {
            error!("An event was passed to the synth, but the soundfont has not been loaded!");
            return;
        };
        // TODO: refacctor midix synth
        let mut synth = synth.lock().unwrap();
        let channel = event.channel().to_byte() as i32;
        let command = (event.status() & 0xF0) as i32;
        let data1 = event.data_1_byte() as i32;
        let data2 = event.data_2_byte().unwrap_or(0) as i32;
        synth.process_midi_message(channel, command, data1, data2);
    }
    /// Returns true if the sound font has been loaded!
    pub fn is_ready(&self) -> bool {
        matches!(self.synthesizer, SynthState::Loaded(_))
    }

    /// Provide a handle to the soundfont file
    pub fn use_soundfont(&mut self, sound_font: Handle<SoundFont>) {
        self.synthesizer = SynthState::LoadHandle { sound_font };
        self._device = None;
    }
}

impl Default for Synth {
    fn default() -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 441,
        };
        Self {
            params,
            synthesizer: SynthState::NotLoaded,
            _device: None,
        }
    }
}

fn spawn_if_not_inserted(mut commands: Commands, synth: Option<Res<Synth>>) {
    if synth.is_some() {
        return;
    }
    commands.init_resource::<Synth>();
}

fn load_audio_font(mut synth: ResMut<Synth>, assets: Res<Assets<SoundFont>>) {
    let SynthState::LoadHandle { sound_font } = &synth.synthesizer else {
        warn!("loading the audio font is out of sync. This is an issue with bevy_midix. Please file an issue!");
        return;
    };
    let Some(sound_font) = assets.get(sound_font) else {
        return;
    };

    let sound_font = Arc::clone(&sound_font.file);

    let synth_settings = SynthesizerSettings::new(synth.params.sample_rate as i32);

    let synthesizer = Arc::new(Mutex::new(
        Synthesizer::new(&sound_font, &synth_settings).unwrap(),
    ));

    let device_synth_ref = synthesizer.clone();

    let mut left = vec![0f32; synth.params.channel_sample_count];
    let mut right = vec![0f32; synth.params.channel_sample_count];

    let _device = run_output_device(synth.params, {
        move |data| {
            let mut synth = device_synth_ref.lock().unwrap();

            synth.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();
    synth.synthesizer = SynthState::Loaded(synthesizer);
    synth._device = Some(Mutex::new(_device));
}

fn state_out_of_sync(synth: Res<Synth>, current_state: Res<State<SynthStatus>>) -> bool {
    &synth.status_should_be() != current_state.get()
}

fn sync_states(synth: Res<Synth>, mut next_state: ResMut<NextState<SynthStatus>>) {
    next_state.set(synth.status_should_be());
}
