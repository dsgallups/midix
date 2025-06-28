mod common;

use common::*;

#[test]
fn verify_no_chirp_on_note_off() {
    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 1, // Single sample for precise testing
        epsilon: 1e-5,
        verbose: true,
        max_differences_to_report: 20,
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play a note
    synth.note_on(0, 60, 100);

    // Let the note stabilize
    let _ = synth.render_and_compare_frames(1000);

    // Get the last sample before note_off
    let before_result = synth.render_and_compare();
    let last_sample_before = synth.mleft[0];

    // Note off
    synth.note_off(0, 60);

    // Get the first sample after note_off
    let after_result = synth.render_and_compare();
    let first_sample_after = synth.mleft[0];

    // Check for discontinuity
    let discontinuity = (first_sample_after - last_sample_before).abs();

    println!("\n=== Chirp Detection Test ===");
    println!("Last sample before note_off: {:.9}", last_sample_before);
    println!("First sample after note_off: {:.9}", first_sample_after);
    println!("Discontinuity: {:.9}", discontinuity);

    // The discontinuity should be small (no abrupt cutoff)
    assert!(
        discontinuity < 0.01,
        "Large discontinuity detected: {}. This would cause a chirp!",
        discontinuity
    );

    // Both synthesizers should match
    assert!(
        before_result.passed && after_result.passed,
        "Synthesizers don't match during note_off transition"
    );
}

#[test]
fn verify_envelope_continues_during_release() {
    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 64,
        epsilon: 1e-5,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play a note
    synth.note_on(0, 60, 100);

    // Let it play for a bit
    let _ = synth.render_and_compare_frames(10);

    // Note off
    synth.note_off(0, 60);

    // Track the envelope decay
    let mut all_zero_frames = 0;
    let mut non_zero_frames = 0;
    let mut max_amplitude = 0.0f32;

    for frame in 0..50 {
        let result = synth.render_and_compare();

        // Check if midix is producing non-zero output
        let frame_has_sound = synth.mleft.iter().any(|&s| s.abs() > 1e-6);

        if frame_has_sound {
            non_zero_frames += 1;
            let frame_max = synth.mleft.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
            max_amplitude = max_amplitude.max(frame_max);
        } else {
            all_zero_frames += 1;
        }

        // The synthesizers should match
        assert!(
            result.passed,
            "Synthesizers diverged during release at frame {}",
            frame
        );
    }

    println!("\n=== Release Phase Analysis ===");
    println!("Frames with audio: {}", non_zero_frames);
    println!("Silent frames: {}", all_zero_frames);
    println!("Max amplitude during release: {:.9}", max_amplitude);

    // The envelope should produce sound for multiple frames during release
    assert!(
        non_zero_frames > 10,
        "Envelope released too quickly. Only {} frames had audio.",
        non_zero_frames
    );
}

#[test]
fn verify_envelope_value_tracking() {
    // This test verifies that the envelope value is properly tracked
    // throughout the note lifecycle

    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 1, // Single sample
        epsilon: 1e-6,
        verbose: false,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play a note
    synth.note_on(0, 60, 100);

    // Track envelope progression
    let mut envelope_values = Vec::new();

    // During attack/decay/sustain
    for _ in 0..1000 {
        let result = synth.render_and_compare();
        envelope_values.push(synth.mleft[0]);
        assert!(
            result.passed,
            "Synthesizers don't match during sustain phase"
        );
    }

    // Find the approximate sustain level
    let sustain_level = envelope_values[900..1000]
        .iter()
        .map(|v| v.abs())
        .sum::<f32>()
        / 100.0;

    println!("\n=== Envelope Tracking Test ===");
    println!("Approximate sustain level: {:.9}", sustain_level);

    // Note off
    synth.note_off(0, 60);

    // The first sample after note_off should start from near the sustain level
    let result = synth.render_and_compare();
    let release_start = synth.mleft[0].abs();

    println!("Release started at: {:.9}", release_start);
    println!("Ratio to sustain: {:.3}", release_start / sustain_level);

    // The release should start from a value close to the sustain level
    assert!(
        (release_start / sustain_level - 1.0).abs() < 0.1,
        "Release didn't start from the correct envelope level. Expected ~{}, got {}",
        sustain_level,
        release_start
    );

    assert!(result.passed, "Synthesizers don't match at release start");
}

#[test]
fn verify_multiple_note_releases() {
    // Test that multiple notes can release properly without affecting each other

    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 64,
        epsilon: 1e-5,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play a chord
    synth.note_on(0, 60, 100); // C
    synth.note_on(0, 64, 100); // E
    synth.note_on(0, 67, 100); // G

    // Let it stabilize
    let _ = synth.render_and_compare_frames(10);

    // Release notes one by one
    synth.note_off(0, 60);

    // Check that we still have sound (from the other notes)
    let result = synth.render_and_compare();
    let has_sound = synth.mleft.iter().any(|&s| s.abs() > 0.001);
    assert!(
        has_sound,
        "All sound stopped when only one note was released!"
    );
    assert!(
        result.passed,
        "Synthesizers don't match after first note off"
    );

    // Continue for a few frames
    let _ = synth.render_and_compare_frames(5);

    // Release second note
    synth.note_off(0, 64);

    let result = synth.render_and_compare();
    let has_sound = synth.mleft.iter().any(|&s| s.abs() > 0.001);
    assert!(
        has_sound,
        "All sound stopped when only two notes were released!"
    );
    assert!(
        result.passed,
        "Synthesizers don't match after second note off"
    );

    // Continue for a few frames
    let _ = synth.render_and_compare_frames(5);

    // Release final note
    synth.note_off(0, 67);

    // Now all notes are in release phase
    let result = synth.render_and_compare_frames(20);
    assert!(
        result.passed,
        "Synthesizers don't match during final release phase"
    );
}

#[test]
fn verify_quick_note_on_off() {
    // Test rapid note on/off sequences

    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 32,
        epsilon: 1e-5,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Rapid fire notes
    for i in 0..10 {
        synth.note_on(0, 60 + i, 100);
        let _ = synth.render_and_compare_frames(2);
        synth.note_off(0, 60 + i);
        let result = synth.render_and_compare_frames(2);

        assert!(
            result.passed,
            "Synthesizers don't match during rapid note {}/{}",
            i + 1,
            10
        );
    }
}

#[test]
fn verify_extreme_release_times() {
    // Test with different instrument programs that might have different release times

    let config = ComparisonConfig {
        sample_rate: 44100,
        frames_per_render: 64,
        epsilon: 1e-5,
        verbose: false,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Test several different programs
    let programs = [0, 1, 8, 16, 24, 32, 40, 48];

    for program in programs {
        synth.reset();
        synth.program_change(0, program);

        // Play note
        synth.note_on(0, 60, 100);
        let _ = synth.render_and_compare_frames(5);

        // Note off
        synth.note_off(0, 60);

        // Check release phase
        let result = synth.render_and_compare_frames(20);
        assert!(
            result.passed,
            "Synthesizers don't match for program {} during release",
            program
        );
    }
}
