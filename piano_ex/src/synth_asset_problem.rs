use bevy::prelude::*;
use bevy_midix::asset::{SoundFont, SoundFontLoader};
use itertools::Itertools;
use midix_synth::prelude::*;
use std::{
    fs::File,
    sync::{Arc, Mutex},
};
use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};

pub fn plugin(app: &mut App) {
    app.init_asset::<SoundFont>()
        .init_asset_loader::<SoundFontLoader>()
        .add_systems(Startup, init_soundfont)
        .add_systems(Update, spawn_synth);
}

#[derive(Resource)]
pub struct Synth {
    sf_handle: Handle<SoundFont>,
    params: OutputDeviceParameters,
    left_buf: Arc<Mutex<Vec<f32>>>,
    right_buf: Arc<Mutex<Vec<f32>>>,
    synthesizer: Option<Arc<Mutex<Synthesizer>>>,
    _device: Option<Arc<Mutex<OutputDevice>>>,
}

impl Synth {
    fn new(sf_handle: Handle<SoundFont>) -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 4410,
        };

        Self {
            sf_handle,
            params,
            left_buf: Arc::new(Mutex::new(vec![0f32; params.channel_sample_count])),
            right_buf: Arc::new(Mutex::new(vec![0f32; params.channel_sample_count])),
            synthesizer: None,
            _device: None,
        }
    }
    fn sf_handle(&self) -> &Handle<SoundFont> {
        &self.sf_handle
    }
    fn initialized(&self) -> bool {
        self.synthesizer.is_some()
    }

    /// Returns some if the sound font is ready
    fn create_synth(&mut self, asset_server: Res<Assets<SoundFont>>) -> bool {
        let Some(sf) = asset_server.get(&self.sf_handle) else {
            return false;
        };

        let synth_settings = SynthesizerSettings::new(self.params.sample_rate as i32);

        let synthesizer = Arc::new(Mutex::new(
            Synthesizer::new(sf.font(), &synth_settings).unwrap(),
        ));

        let device_synth_ref = synthesizer.clone();
        self.synthesizer = Some(synthesizer);

        let left = self.left_buf.clone();
        let right = self.right_buf.clone();

        let _device = run_output_device(self.params, {
            move |data| {
                let mut synth = device_synth_ref.lock().unwrap();
                let mut left = left.lock().unwrap();
                let mut right = right.lock().unwrap();

                synth.render(&mut left[..], &mut right[..]);
                for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                    data[i] = *value;
                }
            }
        })
        .unwrap();

        self._device = Some(Arc::new(Mutex::new(_device)));
        true
    }
}

fn init_soundfont(mut commands: Commands, res: Res<AssetServer>) {
    // hack until I figure out how to create a Handle<AudioFont>
    //
    let sound_font: Handle<SoundFont> = res.load("essential.sf2");

    let synth = Synth::new(sound_font);

    commands.insert_resource(synth);
    // let synth = Synth::new(sound_font.clone());
}

fn spawn_synth(
    mut synth: ResMut<Synth>,
    sound_fonts: Res<Assets<SoundFont>>,
    all_resources: Res<AssetServer>,
) {
    if synth.initialized() {
        let state = all_resources.load_state(synth.sf_handle());
        println!("state: {:?}", state);
        return;
    }

    if synth.create_synth(sound_fonts) {
        println!("spawned!");
    }
}
