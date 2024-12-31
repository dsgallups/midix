use bevy::prelude::*;
use bevy_midix::prelude::ChannelVoiceMessage;
use itertools::Itertools;
use midix_synth::prelude::*;
use std::sync::{Arc, Mutex};
use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};

pub fn plugin(app: &mut App) {
    app.init_resource::<Synth>()
        .add_systems(Update, spawn_synth);
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct Synth {
    params: OutputDeviceParameters,
    left: Arc<Mutex<Vec<f32>>>,
    right: Arc<Mutex<Vec<f32>>>,
    synthesizer: Arc<Mutex<Synthesizer>>,
    _device: Mutex<OutputDevice>,
}

impl Synth {
    pub fn handle_event(&mut self, event: ChannelVoiceMessage<'_>) {
        let channel = event.channel().value() as i32;
        let command = (event.status() & 0xF0) as i32;
        let data1 = *event.data_1_byte() as i32;
        let data2 = *event.data_2_byte().unwrap_or(&0) as i32;

        let mut synth = self.synthesizer.lock().unwrap();
        let mut left = self.left.lock().unwrap();
        let mut right = self.right.lock().unwrap();
        synth.render(&mut left[..], &mut right[..]);
        synth.process_midi_message(channel, command, data1, data2);
    }
}

impl Default for Synth {
    fn default() -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 4410,
        };

        let mut sf2 = include_bytes!("../assets/essential.sf2").as_slice();

        let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());
        let synth_settings = SynthesizerSettings::new(params.sample_rate as i32);

        let synthesizer = Arc::new(Mutex::new(
            Synthesizer::new(&sound_font, &synth_settings).unwrap(),
        ));

        let device_synth_ref = synthesizer.clone();

        let mut left = Arc::new(Mutex::new(vec![0f32; params.channel_sample_count]));
        let mut right = Arc::new(Mutex::new(vec![0f32; params.channel_sample_count]));

        let lc = left.clone();
        let rc = right.clone();
        let _device = run_output_device(params, {
            move |data| {
                let mut synth = device_synth_ref.lock().unwrap();
                let mut left = lc.lock().unwrap();
                let mut right = rc.lock().unwrap();

                for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                    data[i] = *value;
                }
            }
        })
        .unwrap();

        Self {
            params,
            left,
            right,
            synthesizer,
            _device: Mutex::new(_device),
        }
    }
}

fn spawn_synth() {}
