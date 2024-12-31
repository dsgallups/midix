use itertools::Itertools as _;
use midix_synth::prelude::*;
use std::sync::{Arc, Mutex};
use tinyaudio::prelude::*;

fn main() {
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 441,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut sf2 = include_bytes!("../essential.sf2").as_slice();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    println!("midi file loaded");
    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Arc::new(Mutex::new(
        Synthesizer::new(&sound_font, &settings).unwrap(),
    ));

    println!("synth loaded");

    let s_c = synthesizer.clone();

    std::thread::spawn(move || {
        for i in 0..10 {
            let key = 40 + i;

            let mut synth = s_c.lock().unwrap();
            synth.note_on(1, key, 127);
            drop(synth);

            std::thread::sleep(std::time::Duration::from_secs(4));
        }
    });

    // Play the MIDI file.

    println!("sequencer playing");
    // Start the audio output.
    let mut count = 0;
    let _device = run_output_device(params, {
        move |data| {
            println!("closure call: {}", count);
            count += 1;
            let mut synth = synthesizer.lock().unwrap();

            synth.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();

    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(180));
}
