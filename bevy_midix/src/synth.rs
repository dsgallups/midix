use crate::prelude::*;
use bevy::prelude::*;
use itertools::Itertools;
use midix_synth::prelude::*;
use std::sync::{Arc, Mutex};
use tinyaudio::{run_output_device, OutputDevice, OutputDeviceParameters};

pub fn plugin(app: &mut App) {
    app.init_resource::<Synth>()
        .add_systems(Update, spawn_synth);
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct Synth {
    params: OutputDeviceParameters,
    synthesizer: Arc<Mutex<Synthesizer>>,
    _device: Mutex<OutputDevice>,
}

impl Synth {
    pub fn handle_event(&mut self, event: ChannelVoiceMessage) {
        warn!("Event received: {:?}", event);
        // TODO: refacctor midix synth
        let mut synth = self.synthesizer.lock().unwrap();
        let channel = event.channel().to_byte() as i32;
        let command = (event.status() & 0xF0) as i32;
        let data1 = event.data_1_byte().value() as i32;
        let data2 = event.data_2_byte().map(|b| b.value()).unwrap_or(0) as i32;
        synth.process_midi_message(channel, command, data1, data2);
    }
}

impl Default for Synth {
    fn default() -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 441,
        };

        let mut sf2 = include_bytes!("../assets/soundfont.sf2").as_slice();

        let sound_font = Arc::new(midix_synth::soundfont::SoundFont::new(&mut sf2).unwrap());
        let synth_settings = SynthesizerSettings::new(params.sample_rate as i32);

        let synthesizer = Arc::new(Mutex::new(
            Synthesizer::new(&sound_font, &synth_settings).unwrap(),
        ));

        let device_synth_ref = synthesizer.clone();

        let mut left = vec![0f32; params.channel_sample_count];
        let mut right = vec![0f32; params.channel_sample_count];

        let _device = run_output_device(params, {
            move |data| {
                let mut synth = device_synth_ref.lock().unwrap();

                synth.render(&mut left[..], &mut right[..]);
                for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                    data[i] = *value;
                }
            }
        })
        .unwrap();

        Self {
            params,
            synthesizer,
            _device: Mutex::new(_device),
        }
    }
}

fn spawn_synth() {}
