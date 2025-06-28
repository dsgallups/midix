use std::{fs, io::Cursor, sync::Arc};

use midix::prelude::*;

#[test]
fn compare_waveforms() {
    let bytes = fs::read("assets/8bitsf.SF2").unwrap();

    let midix_soundfont = SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();
    let rs_soundfont = rustysynth::SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();

    // feel free to change these values, but ensure the two synths use the same values.
    let sample_rate = 41000;

    let mut midix_synth = Synthesizer::new(
        Arc::new(midix_soundfont),
        &SynthesizerSettings::new(sample_rate),
    )
    .unwrap();

    let mut rs_synth = rustysynth::Synthesizer::new(
        &Arc::new(rs_soundfont),
        &rustysynth::SynthesizerSettings::new(sample_rate),
    )
    .unwrap();

    let channel = Channel::One;
    let key = key!(C, 4);
    let velocity = 127;
    midix_synth.note_on(channel.to_byte(), key.byte(), velocity);
    rs_synth.note_on(channel.to_byte() as i32, key.byte() as i32, velocity as i32);

    const FRAMES: usize = 512;

    const MAX_RENDER: usize = 10;

    let mut mleft = [0_f32; FRAMES];
    let mut mright = [0_f32; FRAMES];

    let mut rleft = [0_f32; FRAMES];
    let mut rright = [0_f32; FRAMES];

    for _ in 0..MAX_RENDER {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        for ((i, v1), v2) in mleft.iter().enumerate().zip(rleft) {
            println!("note_on left I: {i}");
            assert_eq!(*v1, v2);
        }
        for ((i, v1), v2) in mright.iter().enumerate().zip(rright) {
            println!("note_on right I: {i}");
            assert_eq!(*v1, v2);
        }
    }
    midix_synth.note_off(channel.to_byte(), key.byte());
    rs_synth.note_off(channel.to_byte() as i32, key.byte() as i32);
    for _ in 0..MAX_RENDER {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        for ((i, v1), v2) in mleft.iter().enumerate().zip(rleft) {
            println!("note_off left I: {i}");
            assert_eq!(*v1, v2);
        }
        for ((i, v1), v2) in mright.iter().enumerate().zip(rright) {
            println!("note_off right I: {i}");
            assert_eq!(*v1, v2);
        }
    }
}
