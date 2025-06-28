use std::{fs, io::Cursor, sync::Arc};

use midix::prelude::*;

#[test]
fn debug_note_off_behavior() {
    let bytes = fs::read("assets/8bitsf.SF2").unwrap();

    let midix_soundfont = SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();
    let rs_soundfont = rustysynth::SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();

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

    println!("=== Starting note_on ===");
    midix_synth.note_on(channel.to_byte(), key.byte(), velocity);
    rs_synth.note_on(channel.to_byte() as i32, key.byte() as i32, velocity as i32);

    const FRAMES: usize = 64; // Smaller frame size for more granular debugging
    const FRAMES_BEFORE_OFF: usize = 5;
    const FRAMES_AFTER_OFF: usize = 10;

    let mut mleft = [0_f32; FRAMES];
    let mut mright = [0_f32; FRAMES];
    let mut rleft = [0_f32; FRAMES];
    let mut rright = [0_f32; FRAMES];

    // Render a few frames before note_off
    println!("\n=== Rendering {FRAMES_BEFORE_OFF} frames before note_off ===",);
    for frame_idx in 0..FRAMES_BEFORE_OFF {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        println!("\nFrame {frame_idx} (before note_off):");
        println!("  First 10 samples:");
        for i in 0..10.min(FRAMES) {
            let diff = (mleft[i] - rleft[i]).abs();
            let match_str = if diff < 0.0001 { "✓" } else { "✗" };
            println!(
                "    [{:3}] midix: {:+.8}, rusty: {:+.8}, diff: {:+.8e} {}",
                i, mleft[i], rleft[i], diff, match_str
            );
        }

        // Check if outputs match
        let max_diff = mleft
            .iter()
            .zip(rleft.iter())
            .map(|(m, r)| (m - r).abs())
            .fold(0.0f32, f32::max);
        println!("  Max difference in frame: {max_diff:.8e}");
    }

    // Call note_off
    println!("\n=== Calling note_off ===");
    midix_synth.note_off(channel.to_byte(), key.byte());
    rs_synth.note_off(channel.to_byte() as i32, key.byte() as i32);

    // Render frames after note_off
    println!("\n=== Rendering {FRAMES_AFTER_OFF} frames after note_off ===",);
    for frame_idx in 0..FRAMES_AFTER_OFF {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        println!("\nFrame {frame_idx} (after note_off):");

        // Print more samples for the first frame after note_off
        let samples_to_print = if frame_idx == 0 {
            20.min(FRAMES)
        } else {
            10.min(FRAMES)
        };

        println!("  First {samples_to_print} samples:");
        for i in 0..samples_to_print {
            let diff = (mleft[i] - rleft[i]).abs();
            let match_str = if diff < 0.0001 { "✓" } else { "✗" };
            println!(
                "    [{:3}] midix: {:+.8}, rusty: {:+.8}, diff: {:+.8e} {}",
                i, mleft[i], rleft[i], diff, match_str
            );
        }

        // Check if outputs match
        let max_diff = mleft
            .iter()
            .zip(rleft.iter())
            .map(|(m, r)| (m - r).abs())
            .fold(0.0f32, f32::max);
        let avg_diff = mleft
            .iter()
            .zip(rleft.iter())
            .map(|(m, r)| (m - r).abs())
            .sum::<f32>()
            / FRAMES as f32;

        println!("  Max difference in frame: {max_diff:.8e}");
        println!("  Avg difference in frame: {avg_diff:.8e}");

        // Find first mismatching sample
        if let Some((idx, (m, r))) = mleft
            .iter()
            .zip(rleft.iter())
            .enumerate()
            .find(|(_, (m, r))| (*m - *r).abs() > 0.0001)
        {
            println!("  First mismatch at sample {idx}: midix={m:+.8}, rusty={r:+.8}",);
        }

        // Print some statistics
        let midix_rms = (mleft.iter().map(|x| x * x).sum::<f32>() / FRAMES as f32).sqrt();
        let rusty_rms = (rleft.iter().map(|x| x * x).sum::<f32>() / FRAMES as f32).sqrt();
        println!("  RMS - midix: {midix_rms:.8}, rusty: {rusty_rms:.8}");

        if frame_idx == 0 {
            // Additional debugging for the first frame after note_off
            println!("\n  === Detailed analysis of first frame after note_off ===");

            // Check for sudden changes
            if FRAMES_BEFORE_OFF > 0 {
                println!("  Looking for sudden amplitude changes...");
                // We'd need to save the last frame before note_off for this comparison
            }

            // Check if either synth is producing silence
            let midix_silent = mleft.iter().all(|&x| x.abs() < 0.00001);
            let rusty_silent = rleft.iter().all(|&x| x.abs() < 0.00001);
            println!("  Midix producing silence: {midix_silent}");
            println!("  Rusty producing silence: {rusty_silent}");
        }
    }

    println!("\n=== Summary ===");
    println!("The test shows differences in how the two synthesizers handle note_off.");
    println!("This is likely related to envelope release behavior or voice state transitions.");
}

#[test]
fn debug_single_sample_after_note_off() {
    let bytes = fs::read("assets/8bitsf.SF2").unwrap();

    let midix_soundfont = SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();
    let rs_soundfont = rustysynth::SoundFont::new(&mut Cursor::new(bytes.clone())).unwrap();

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

    // Render some frames to let the note play
    let mut buffer_left = [0_f32; 512];
    let mut buffer_right = [0_f32; 512];
    for _ in 0..10 {
        midix_synth.render(&mut buffer_left[..], &mut buffer_right[..]);
        rs_synth.render(&mut buffer_left[..], &mut buffer_right[..]);
    }

    // Get the last frame before note_off
    let mut mlast_left = [0_f32; 1];
    let mut mlast_right = [0_f32; 1];
    let mut rlast_left = [0_f32; 1];
    let mut rlast_right = [0_f32; 1];
    midix_synth.render(&mut mlast_left[..], &mut mlast_right[..]);
    rs_synth.render(&mut rlast_left[..], &mut rlast_right[..]);

    println!("Last sample before note_off:");
    println!("  midix: {:+.8}", mlast_left[0]);
    println!("  rusty: {:+.8}", rlast_left[0]);

    // Note off
    midix_synth.note_off(channel.to_byte(), key.byte());
    rs_synth.note_off(channel.to_byte() as i32, key.byte() as i32);

    // Get the first sample after note_off
    let mut mfirst_left = [0_f32; 1];
    let mut mfirst_right = [0_f32; 1];
    let mut rfirst_left = [0_f32; 1];
    let mut rfirst_right = [0_f32; 1];
    midix_synth.render(&mut mfirst_left[..], &mut mfirst_right[..]);
    rs_synth.render(&mut rfirst_left[..], &mut rfirst_right[..]);

    println!("\nFirst sample after note_off:");
    println!("  midix: {:+.8}", mfirst_left[0]);
    println!("  rusty: {:+.8}", rfirst_left[0]);

    println!("\nTransition:");
    println!(
        "  midix: {:+.8} -> {:+.8} (delta: {:+.8})",
        mlast_left[0],
        mfirst_left[0],
        mfirst_left[0] - mlast_left[0]
    );
    println!(
        "  rusty: {:+.8} -> {:+.8} (delta: {:+.8})",
        rlast_left[0],
        rfirst_left[0],
        rfirst_left[0] - rlast_left[0]
    );

    let discontinuity_midix = (mfirst_left[0] - mlast_left[0]).abs();
    let discontinuity_rusty = (rfirst_left[0] - rlast_left[0]).abs();

    println!("\nDiscontinuity magnitude:");
    println!("  midix: {discontinuity_midix:.8}");
    println!("  rusty: {discontinuity_rusty:.8}");

    if discontinuity_midix > 0.01 || discontinuity_rusty > 0.01 {
        println!("\nWARNING: Large discontinuity detected! This could cause the 'chirp' sound.");
    }
}
