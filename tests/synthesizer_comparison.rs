mod common;

use common::*;

#[test]
fn test_basic_note_on_off() {
    let config = ComparisonConfig {
        epsilon: 5e-3, // Allow small floating point differences
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    let mut scenario = TestScenario::note_on_off(
        0,   // channel
        60,  // middle C
        100, // velocity
        10,  // frames before note off
        10,  // frames after note off
    );

    let result = scenario.run(&mut synth);

    assert!(
        result.passed,
        "Basic note on/off test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_pitch_bend() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Test pitch bend up
    let mut scenario = TestScenario::pitch_bend(
        0,     // channel
        60,    // middle C
        100,   // velocity
        12288, // bend up (max is 16383, center is 8192)
        5,     // frames before bend
        10,    // frames after bend
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Pitch bend up test failed with max difference: {:.9e}",
        result.max_difference
    );

    // Test pitch bend down
    let mut scenario = TestScenario::pitch_bend(
        0,    // channel
        60,   // middle C
        100,  // velocity
        4096, // bend down
        5,    // frames before bend
        10,   // frames after bend
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Pitch bend down test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_volume_control() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Test volume change
    let mut scenario = TestScenario::controller_change(
        0,   // channel
        60,  // middle C
        100, // velocity
        7,   // volume controller
        64,  // half volume
        5,   // frames before change
        10,  // frames after change
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Volume control test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_pan_control() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Test pan hard left
    let mut scenario = TestScenario::controller_change(
        0,   // channel
        60,  // middle C
        100, // velocity
        10,  // pan controller
        0,   // hard left
        5,   // frames before change
        10,  // frames after change
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Pan left test failed with max difference: {:.9e}",
        result.max_difference
    );

    // Test pan hard right
    let mut scenario = TestScenario::controller_change(
        0,   // channel
        60,  // middle C
        100, // velocity
        10,  // pan controller
        127, // hard right
        5,   // frames before change
        10,  // frames after change
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Pan right test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_sustain_pedal() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Custom scenario for sustain pedal
    synth.reset();

    // Note on
    synth.note_on(0, 60, 100);

    // Render a few frames
    let _ = synth.render_and_compare_frames(5);

    // Press sustain pedal
    synth.controller(0, 64, 127);

    // Note off (but should continue sounding due to sustain)
    synth.note_off(0, 60);

    // Render and check that sound continues
    let result = synth.render_and_compare_frames(5);
    assert!(
        result.passed,
        "Sustain pedal test (pedal on) failed with max difference: {:.9e}",
        result.max_difference
    );

    // Release sustain pedal
    synth.controller(0, 64, 0);

    // Now the note should start releasing
    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Sustain pedal test (pedal off) failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_modulation_wheel() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    let mut scenario = TestScenario::controller_change(
        0,   // channel
        60,  // middle C
        100, // velocity
        1,   // modulation wheel
        64,  // half modulation
        5,   // frames before change
        10,  // frames after change
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Modulation wheel test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_multiple_notes() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
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

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Chord test (note on) failed with max difference: {:.9e}",
        result.max_difference
    );

    // Release one note
    synth.note_off(0, 64);

    let result = synth.render_and_compare_frames(5);
    assert!(
        result.passed,
        "Chord test (partial release) failed with max difference: {:.9e}",
        result.max_difference
    );

    // Release remaining notes
    synth.note_off(0, 60);
    synth.note_off(0, 67);

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Chord test (full release) failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_percussion_channel() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    // Channel 9 (index 9) is typically the percussion channel
    let mut scenario = TestScenario::note_on_off(
        9,   // percussion channel
        36,  // kick drum
        100, // velocity
        5,   // frames before note off
        5,   // frames after note off
    );

    let result = scenario.run(&mut synth);
    assert!(
        result.passed,
        "Percussion channel test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_program_change() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Change to a different instrument
    synth.program_change(0, 1); // Change to program 1

    // Play a note with the new instrument
    synth.note_on(0, 60, 100);

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Program change test (note with new program) failed with max difference: {:.9e}",
        result.max_difference
    );

    synth.note_off(0, 60);

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Program change test (release) failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_all_notes_off() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play multiple notes
    synth.note_on(0, 60, 100);
    synth.note_on(0, 64, 100);
    synth.note_on(0, 67, 100);

    let _ = synth.render_and_compare_frames(5);

    // All notes off controller
    synth.controller(0, 123, 0);

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "All notes off test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
fn test_reset_all_controllers() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: true,
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Set various controllers
    synth.controller(0, 1, 127); // modulation
    synth.controller(0, 7, 64); // volume
    synth.controller(0, 10, 0); // pan
    synth.pitch_bend(0, 16383); // max pitch bend

    // Play a note
    synth.note_on(0, 60, 100);
    let _ = synth.render_and_compare_frames(5);

    // Reset all controllers
    synth.controller(0, 121, 0);

    let result = synth.render_and_compare_frames(10);
    assert!(
        result.passed,
        "Reset all controllers test failed with max difference: {:.9e}",
        result.max_difference
    );
}

#[test]
#[ignore] // This test can be slow
fn test_stress_many_notes() {
    let config = ComparisonConfig {
        epsilon: 5e-3,
        verbose: false, // Less verbose for stress test
        ..Default::default()
    };

    let mut synth = SynthesizerComparison::new("assets/8bitsf.SF2", config)
        .expect("Failed to create synthesizer comparison");

    synth.reset();

    // Play many notes across different channels
    for channel in 0..8 {
        for note in (40..80).step_by(3) {
            synth.note_on(channel, note, 80);
        }
    }

    let result = synth.render_and_compare_frames(20);
    assert!(
        result.passed,
        "Stress test (many notes) failed with max difference: {:.9e}",
        result.max_difference
    );

    // Release all notes
    for channel in 0..8 {
        for note in (40..80).step_by(3) {
            synth.note_off(channel, note);
        }
    }

    let result = synth.render_and_compare_frames(20);
    assert!(
        result.passed,
        "Stress test (release) failed with max difference: {:.9e}",
        result.max_difference
    );
}
