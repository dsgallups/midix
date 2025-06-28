use std::{fs, io::Cursor, sync::Arc};

use midix::prelude::*;

const EPSILON: f32 = 1e-6; // Tolerance for floating point comparison

#[test]
fn compare_waveforms_with_tolerance() {
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

    const FRAMES: usize = 512;
    const MAX_RENDER: usize = 10;

    let mut mleft = [0_f32; FRAMES];
    let mut mright = [0_f32; FRAMES];
    let mut rleft = [0_f32; FRAMES];
    let mut rright = [0_f32; FRAMES];

    let mut max_diff_note_on = 0_f32;
    let mut total_samples_note_on = 0;
    let mut differences_note_on = Vec::new();

    // Test note_on phase
    for frame_idx in 0..MAX_RENDER {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        for (i, (m, r)) in mleft.iter().zip(rleft.iter()).enumerate() {
            let diff = (m - r).abs();
            if diff > EPSILON {
                differences_note_on.push((frame_idx * FRAMES + i, *m, *r, diff));
            }
            max_diff_note_on = max_diff_note_on.max(diff);
            total_samples_note_on += 1;
        }
    }

    // Note off
    midix_synth.note_off(channel.to_byte(), key.byte());
    rs_synth.note_off(channel.to_byte() as i32, key.byte() as i32);

    let mut max_diff_note_off = 0_f32;
    let mut total_samples_note_off = 0;
    let mut differences_note_off = Vec::new();

    // Test note_off phase
    for frame_idx in 0..MAX_RENDER {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        for (i, (m, r)) in mleft.iter().zip(rleft.iter()).enumerate() {
            let diff = (m - r).abs();
            if diff > EPSILON {
                differences_note_off.push((frame_idx * FRAMES + i, *m, *r, diff));
            }
            max_diff_note_off = max_diff_note_off.max(diff);
            total_samples_note_off += 1;
        }
    }

    // Print statistics
    println!("\n=== Note On Phase Statistics ===");
    println!("Total samples compared: {total_samples_note_on}");
    println!("Maximum difference: {max_diff_note_on:.9e}");
    println!(
        "Samples exceeding epsilon ({}): {}",
        EPSILON,
        differences_note_on.len()
    );

    if !differences_note_on.is_empty() {
        println!("\nFirst 10 differences during note_on:");
        for (idx, (sample_idx, midix_val, rusty_val, diff)) in
            differences_note_on.iter().take(10).enumerate()
        {
            println!(
                "  [{idx}] Sample {sample_idx}: midix={midix_val:.9}, rusty={rusty_val:.9}, diff={diff:.9e}",
            );
        }
    }

    println!("\n=== Note Off Phase Statistics ===");
    println!("Total samples compared: {total_samples_note_off}");
    println!("Maximum difference: {max_diff_note_off:.9e}");
    println!(
        "Samples exceeding epsilon ({}): {}",
        EPSILON,
        differences_note_off.len()
    );

    if !differences_note_off.is_empty() {
        println!("\nFirst 10 differences during note_off:");
        for (idx, (sample_idx, midix_val, rusty_val, diff)) in
            differences_note_off.iter().take(10).enumerate()
        {
            println!(
                "  [{idx}] Sample {sample_idx}: midix={midix_val:.9}, rusty={rusty_val:.9}, diff={diff:.9e}",
            );
        }

        // Find the largest differences
        let mut sorted_diffs = differences_note_off.clone();
        sorted_diffs.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());

        println!("\nTop 10 largest differences during note_off:");
        for (idx, (sample_idx, midix_val, rusty_val, diff)) in
            sorted_diffs.iter().take(10).enumerate()
        {
            println!(
                "  [{idx}] Sample {sample_idx}: midix={midix_val:.9}, rusty={rusty_val:.9}, diff={diff:.9e}",
            );
        }
    }

    // Check if differences are within acceptable range
    let acceptable_epsilon = 1e-5; // A slightly larger tolerance for practical purposes

    if max_diff_note_on < acceptable_epsilon && max_diff_note_off < acceptable_epsilon {
        println!("\n✓ All differences are within acceptable tolerance ({acceptable_epsilon:.9e})",);
    } else {
        println!("\n✗ Some differences exceed acceptable tolerance ({acceptable_epsilon:.9e})",);
        panic!("Waveforms differ by more than acceptable tolerance");
    }
}

#[test]
fn analyze_envelope_differences() {
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

    // Render until note is stable
    let mut buffer_left = [0_f32; 512];
    let mut buffer_right = [0_f32; 512];
    for _ in 0..10 {
        midix_synth.render(&mut buffer_left[..], &mut buffer_right[..]);
        rs_synth.render(&mut buffer_left[..], &mut buffer_right[..]);
    }

    // Note off
    midix_synth.note_off(channel.to_byte(), key.byte());
    rs_synth.note_off(channel.to_byte() as i32, key.byte() as i32);

    // Track envelope over time
    println!("\n=== Envelope Release Analysis ===");

    let mut mleft = [0_f32; 1];
    let mut mright = [0_f32; 1];
    let mut rleft = [0_f32; 1];
    let mut rright = [0_f32; 1];

    for i in 0..100 {
        midix_synth.render(&mut mleft[..], &mut mright[..]);
        rs_synth.render(&mut rleft[..], &mut rright[..]);

        let diff = (mleft[0] - rleft[0]).abs();
        let ratio = if rleft[0].abs() > 1e-10 {
            mleft[0] / rleft[0]
        } else {
            0.0
        };

        if i < 20 || i % 10 == 0 {
            println!(
                "Sample {:3}: midix={:+.9}, rusty={:+.9}, diff={:.9e}, ratio={:.9}",
                i, mleft[0], rleft[0], diff, ratio
            );
        }
    }
}
