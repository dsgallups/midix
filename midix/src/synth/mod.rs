#![doc = r#"
Extensions for `rustysynth` interop
"#]
use crate::prelude::*;
use rustysynth::*;

impl MidiTarget for Synthesizer {
    type Error = ();
    fn handle_event(&mut self, event: MidiMessage) -> Result<(), Self::Error> {
        match event {
            MidiMessage::SysCommon(_s) => {
                //todo
                Ok(())
            }
            MidiMessage::SysRealTime(_s) => {
                //todo
                Ok(())
            }
            MidiMessage::SysExclusive(_s) => {
                //todo
                Ok(())
            }
            MidiMessage::ChannelVoice(cvm) => {
                /*self.process_midi_message(
                    *cvm.channel().byte() as i32,
                    (*cvm.status() & 0xF0) as i32,
                    data1,
                    data2,
                );*/
                //todo
                Ok(())
            }
            MidiMessage::ChannelMode(cm) => {
                //cm.

                //self.process_midi_message(channel, command, data1, data2);
                //todo
                Ok(())
            }
        }
    }
}

#[test]
fn test_synth() {
    use std::fs::File;
    use std::sync::Arc;

    // Load the SoundFont.
    let mut sf2 = include_bytes!("../../essential.sf2").as_slice();

    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = File::open("flourish.mid").unwrap();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(44100);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    // The output buffer.
    let sample_count = (settings.sample_rate as f64 * midi_file.get_length()) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    sequencer.render(&mut left[..], &mut right[..]);
}

#[test]
fn audio_synth() {
    use itertools::Itertools as _;
    use std::sync::Arc;
    use tinyaudio::prelude::*;
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
    let mut sf2 = include_bytes!("../../essential.sf2").as_slice();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = include_bytes!("../../test-asset/Clementi.mid").as_slice();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());
    println!("midi file loaded");
    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    println!("synth loaded");
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    println!("sequencer loaded");
    // Play the MIDI file.
    sequencer.play(&midi_file, false);

    println!("sequencer playing");
    // Start the audio output.
    let mut count = 0;
    let _device = run_output_device(params, {
        move |data| {
            println!("closure call: {}", count);
            count += 1;
            sequencer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();
    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(180));
}

#[test]
fn stream_synth() {
    use itertools::Itertools as _;
    use std::sync::{Arc, Mutex};
    use tinyaudio::prelude::*;
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
    let mut sf2 = include_bytes!("../../essential.sf2").as_slice();
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
