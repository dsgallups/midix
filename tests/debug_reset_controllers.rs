mod common;

use common::*;

fn _debug_reset_all_controllers_behavior() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        frames_per_render: 64,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    println!("\n=== DEBUG: Reset All Controllers Behavior ===\n");

    synth.reset();

    // Set some controllers to non-default values
    println!("1. Setting controllers to non-default values:");
    synth.controller(0, 1, 127); // modulation to max
    synth.controller(0, 7, 64); // volume to half
    synth.controller(0, 10, 0); // pan to hard left
    synth.pitch_bend(0, 16383); // max pitch bend up
    println!("   - Modulation: 127");
    println!("   - Volume: 64");
    println!("   - Pan: 0");
    println!("   - Pitch Bend: 16383 (max)\n");

    // Play a note
    println!("2. Playing note (Middle C, velocity 100)");
    synth.note_on(0, 60, 100);

    // Render some frames to let the note sound
    println!("3. Rendering 5 frames to let note stabilize...");
    let _ = synth.render_and_compare_frames(5);

    // Sample the output before reset
    println!("4. Sampling output before reset:");
    synth
        .midix_synth
        .render(&mut synth.mleft, &mut synth.mright);
    synth
        .rusty_synth
        .render(&mut synth.rleft, &mut synth.rright);

    let midix_before_max = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    let rusty_before_max = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

    println!("   - Midix max amplitude: {midix_before_max:.6}");
    println!("   - RustySynth max amplitude: {rusty_before_max:.6}");
    println!(
        "   - Both producing sound: {}\n",
        midix_before_max > 0.001 && rusty_before_max > 0.001
    );

    // Send Reset All Controllers
    println!("5. Sending Reset All Controllers (CC 121)");
    synth.controller(0, 121, 0);

    // Immediately sample the output after reset
    println!("6. Sampling output immediately after reset:");
    synth
        .midix_synth
        .render(&mut synth.mleft, &mut synth.mright);
    synth
        .rusty_synth
        .render(&mut synth.rleft, &mut synth.rright);

    let midix_after_max = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    let rusty_after_max = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

    println!("   - Midix max amplitude: {midix_after_max:.6}",);
    println!("   - RustySynth max amplitude: {rusty_after_max:.6}");

    if midix_after_max > 0.001 && rusty_after_max < 0.001 {
        println!("   ⚠️  RustySynth stopped producing sound while Midix continues!");
    } else if midix_after_max < 0.001 && rusty_after_max > 0.001 {
        println!("   ⚠️  Midix stopped producing sound while RustySynth continues!");
    } else if midix_after_max < 0.001 && rusty_after_max < 0.001 {
        println!("   ⚠️  Both synthesizers stopped producing sound!");
    } else {
        println!("   ✓  Both synthesizers still producing sound");
    }

    // Check amplitude difference
    let before_diff = (midix_before_max - rusty_before_max).abs();
    let after_diff = (midix_after_max - rusty_after_max).abs();

    println!("\n7. Amplitude analysis:");
    println!("   - Difference before reset: {before_diff:.6}",);
    println!("   - Difference after reset: {after_diff:.6}",);
    println!("   - Change in difference: {:.6}", after_diff - before_diff);

    // Continue rendering for a few more frames
    println!("\n8. Continuing to render 10 more frames...");
    for i in 0..10 {
        synth
            .midix_synth
            .render(&mut synth.mleft, &mut synth.mright);
        synth
            .rusty_synth
            .render(&mut synth.rleft, &mut synth.rright);

        let midix_max = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
        let rusty_max = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

        println!(
            "   Frame {}: Midix={:.6}, RustySynth={:.6}, Diff={:.6}",
            i + 1,
            midix_max,
            rusty_max,
            (midix_max - rusty_max).abs()
        );
    }

    // Send note off to clean up
    println!("\n9. Sending note off");
    synth.note_off(0, 60);

    // Check a few frames of release
    println!("10. Checking release phase (5 frames):");
    for i in 0..5 {
        synth
            .midix_synth
            .render(&mut synth.mleft, &mut synth.mright);
        synth
            .rusty_synth
            .render(&mut synth.rleft, &mut synth.rright);

        let midix_max = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
        let rusty_max = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

        println!(
            "   Frame {}: Midix={:.6}, RustySynth={:.6}",
            i + 1,
            midix_max,
            rusty_max
        );
    }

    println!("\n=== End of Debug Test ===\n");
}

#[test]
fn debug_controller_values_after_reset() {
    // This test checks what controller values are actually reset
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: false,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    println!("\n=== DEBUG: Controller Values After Reset ===\n");

    synth.reset();

    // Set various controllers
    synth.controller(0, 1, 127); // modulation
    synth.controller(0, 7, 64); // volume
    synth.controller(0, 10, 0); // pan
    synth.controller(0, 11, 64); // expression
    synth.controller(0, 64, 127); // hold pedal
    synth.pitch_bend(0, 16383); // pitch bend

    println!("Controllers set to non-default values");

    // Play a note to ensure channel is active
    synth.note_on(0, 60, 100);
    let _ = synth.render_and_compare_frames(2);

    // Reset all controllers
    println!("Sending Reset All Controllers (CC 121)");
    synth.controller(0, 121, 0);

    // Note: We can't directly query controller values from the synthesizers
    // but we can observe their effects

    println!("\nObserving effects of reset:");

    // Test modulation effect
    synth.note_on(0, 62, 100);
    let _ = synth.render_and_compare_frames(5);
    synth.note_off(0, 62);

    // Test with hold pedal
    synth.note_on(0, 64, 100);
    synth.note_off(0, 64);
    let _ = synth.render_and_compare_frames(5);

    println!("Test completed - check output differences");
}

#[test]
fn debug_reset_with_active_voices() {
    // Test reset_all_controllers with multiple active voices
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: false,
        frames_per_render: 32,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    println!("\n=== DEBUG: Reset with Multiple Active Voices ===\n");

    synth.reset();

    // Play multiple notes
    println!("Playing chord: C-E-G");
    synth.note_on(0, 60, 100); // C
    synth.note_on(0, 64, 100); // E
    synth.note_on(0, 67, 100); // G

    // Let them sound
    let _ = synth.render_and_compare_frames(5);

    // Check amplitude before reset
    synth
        .midix_synth
        .render(&mut synth.mleft, &mut synth.mright);
    synth
        .rusty_synth
        .render(&mut synth.rleft, &mut synth.rright);

    let midix_before = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    let rusty_before = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

    println!("Before reset: Midix={midix_before:.6}, RustySynth={rusty_before:.6}",);

    // Reset all controllers
    synth.controller(0, 121, 0);

    // Check amplitude after reset
    synth
        .midix_synth
        .render(&mut synth.mleft, &mut synth.mright);
    synth
        .rusty_synth
        .render(&mut synth.rleft, &mut synth.rright);

    let midix_after = synth.mleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);
    let rusty_after = synth.rleft.iter().map(|&s| s.abs()).fold(0.0f32, f32::max);

    println!("After reset: Midix={midix_after:.6}, RustySynth={rusty_after:.6}",);

    if rusty_after < 0.001 && midix_after > 0.001 {
        println!("\n⚠️  RustySynth appears to stop all voices on reset!");
        println!("This is incorrect behavior - CC 121 should not stop playing notes.");
    }

    // Clean up
    synth.note_off(0, 60);
    synth.note_off(0, 64);
    synth.note_off(0, 67);
}
