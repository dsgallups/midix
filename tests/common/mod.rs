#![allow(dead_code)]

use midix::prelude::*;
use std::{fs, io::Cursor, sync::Arc};

/// Configuration for synthesizer comparison tests
#[derive(Debug, Clone)]
pub struct ComparisonConfig {
    /// Sample rate for both synthesizers
    pub sample_rate: i32,
    /// Number of frames per render call
    pub frames_per_render: usize,
    /// Tolerance for floating point comparison
    pub epsilon: f32,
    /// Whether to print detailed output
    pub verbose: bool,
    /// Maximum number of differences to report
    pub max_differences_to_report: usize,
}

impl Default for ComparisonConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            frames_per_render: 512,
            epsilon: 1e-6,
            verbose: false,
            max_differences_to_report: 10,
        }
    }
}

/// Result of comparing two waveforms
#[derive(Debug)]
pub struct ComparisonResult {
    pub total_samples: usize,
    pub max_difference: f32,
    pub differences: Vec<SampleDifference>,
    pub passed: bool,
}

#[derive(Debug, Clone)]
pub struct SampleDifference {
    pub sample_index: usize,
    pub midix_value: f32,
    pub rusty_value: f32,
    pub difference: f32,
}

/// Test harness for comparing midix and RustySynth
pub struct SynthesizerComparison {
    pub midix_synth: Synthesizer,
    pub rusty_synth: rustysynth::Synthesizer,
    pub config: ComparisonConfig,
    pub mleft: Vec<f32>,
    pub mright: Vec<f32>,
    pub rleft: Vec<f32>,
    pub rright: Vec<f32>,
}

impl SynthesizerComparison {
    /// Create a new comparison harness with the given soundfont and configuration
    pub fn new(
        soundfont_path: &str,
        config: ComparisonConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = fs::read(soundfont_path)?;

        let midix_soundfont = SoundFont::new(&mut Cursor::new(bytes.clone()))?;
        let rs_soundfont = rustysynth::SoundFont::new(&mut Cursor::new(bytes.clone()))?;

        let midix_synth = Synthesizer::new(
            Arc::new(midix_soundfont),
            &SynthesizerSettings::new(config.sample_rate),
        )?;

        let rusty_synth = rustysynth::Synthesizer::new(
            &Arc::new(rs_soundfont),
            &rustysynth::SynthesizerSettings::new(config.sample_rate),
        )?;

        let buffer_size = config.frames_per_render;

        Ok(Self {
            midix_synth,
            rusty_synth,
            config,
            mleft: vec![0.0; buffer_size],
            mright: vec![0.0; buffer_size],
            rleft: vec![0.0; buffer_size],
            rright: vec![0.0; buffer_size],
        })
    }

    /// Note on for both synthesizers
    pub fn note_on(&mut self, channel: u8, key: u8, velocity: u8) {
        self.midix_synth.note_on(channel, key, velocity);
        self.rusty_synth
            .note_on(channel as i32, key as i32, velocity as i32);
    }

    /// Note off for both synthesizers
    pub fn note_off(&mut self, channel: u8, key: u8) {
        self.midix_synth.note_off(channel, key);
        self.rusty_synth.note_off(channel as i32, key as i32);
    }

    /// Process MIDI message for both synthesizers
    #[allow(dead_code)]
    pub fn process_midi_message(&mut self, channel: u8, message: &[u8]) {
        // For midix, we need to handle different message types
        // This is a simplified version - you might want to expand this
        match message.first() {
            Some(&(0x80..=0x8F)) => {
                // Note off
                if let (Some(&key), Some(&_velocity)) = (message.get(1), message.get(2)) {
                    self.note_off(channel, key);
                }
            }
            Some(&(0x90..=0x9F)) => {
                // Note on
                if let (Some(&key), Some(&velocity)) = (message.get(1), message.get(2)) {
                    self.note_on(channel, key, velocity);
                }
            }
            Some(&(0xE0..=0xEF)) => {
                // Pitch bend
                if let (Some(&lsb), Some(&msb)) = (message.get(1), message.get(2)) {
                    self.midix_synth
                        .process_midi_message(0xE0 | channel, lsb, msb);
                    self.rusty_synth.process_midi_message(
                        channel as i32,
                        0xE0,
                        lsb as i32,
                        msb as i32,
                    );
                }
            }
            Some(&(0xB0..=0xBF)) => {
                // Control change
                if let (Some(&controller), Some(&value)) = (message.get(1), message.get(2)) {
                    self.midix_synth
                        .process_midi_message(0xB0 | channel, controller, value);
                    self.rusty_synth.process_midi_message(
                        channel as i32,
                        0xB0,
                        controller as i32,
                        value as i32,
                    );
                }
            }
            Some(&(0xC0..=0xCF)) => {
                // Program change
                if let Some(&program) = message.get(1) {
                    self.midix_synth
                        .process_midi_message(0xC0 | channel, program, 0);
                    self.rusty_synth
                        .process_midi_message(channel as i32, 0xC0, program as i32, 0);
                }
            }
            _ => {}
        }
    }

    /// Set pitch bend for both synthesizers
    pub fn pitch_bend(&mut self, channel: u8, value: u16) {
        let lsb = (value & 0x7F) as u8;
        let msb = ((value >> 7) & 0x7F) as u8;
        self.midix_synth
            .process_midi_message(0xE0 | channel, lsb, msb);
        self.rusty_synth
            .process_midi_message(channel as i32, 0xE0, lsb as i32, msb as i32);
    }

    /// Set controller for both synthesizers
    pub fn controller(&mut self, channel: u8, controller: u8, value: u8) {
        self.midix_synth
            .process_midi_message(0xB0 | channel, controller, value);
        self.rusty_synth.process_midi_message(
            channel as i32,
            0xB0,
            controller as i32,
            value as i32,
        );
    }

    /// Program change for both synthesizers
    pub fn program_change(&mut self, channel: u8, program: u8) {
        self.midix_synth
            .process_midi_message(0xC0 | channel, program, 0);
        self.rusty_synth
            .process_midi_message(channel as i32, 0xC0, program as i32, 0);
    }

    /// Bank select for both synthesizers
    pub fn bank_select(&mut self, channel: u8, bank: u16) {
        // Bank select is typically done with CC 0 (MSB) and CC 32 (LSB)
        let msb = (bank >> 7) as u8;
        let lsb = (bank & 0x7F) as u8;
        self.controller(channel, 0, msb);
        self.controller(channel, 32, lsb);
    }

    /// Reset both synthesizers
    pub fn reset(&mut self) {
        self.midix_synth.reset();
        self.rusty_synth.reset();
    }

    /// Render and compare one frame
    pub fn render_and_compare(&mut self) -> ComparisonResult {
        // Render both synthesizers
        self.midix_synth.render(&mut self.mleft, &mut self.mright);
        self.rusty_synth.render(&mut self.rleft, &mut self.rright);

        // Compare outputs
        self.compare_buffers(&self.mleft, &self.rleft, "left")
    }

    /// Render and compare multiple frames
    pub fn render_and_compare_frames(&mut self, num_frames: usize) -> ComparisonResult {
        let mut all_differences = Vec::new();
        let mut max_difference = 0.0f32;
        let mut total_samples = 0;

        for frame_idx in 0..num_frames {
            // Render both synthesizers
            self.midix_synth.render(&mut self.mleft, &mut self.mright);
            self.rusty_synth.render(&mut self.rleft, &mut self.rright);

            // Compare left channel
            for (i, (m, r)) in self.mleft.iter().zip(self.rleft.iter()).enumerate() {
                let diff = (m - r).abs();
                if diff > self.config.epsilon {
                    all_differences.push(SampleDifference {
                        sample_index: frame_idx * self.config.frames_per_render + i,
                        midix_value: *m,
                        rusty_value: *r,
                        difference: diff,
                    });
                }
                max_difference = max_difference.max(diff);
                total_samples += 1;
            }

            // Compare right channel
            for (i, (m, r)) in self.mright.iter().zip(self.rright.iter()).enumerate() {
                let diff = (m - r).abs();
                if diff > self.config.epsilon {
                    all_differences.push(SampleDifference {
                        sample_index: frame_idx * self.config.frames_per_render
                            + i
                            + self.mleft.len(),
                        midix_value: *m,
                        rusty_value: *r,
                        difference: diff,
                    });
                }
                max_difference = max_difference.max(diff);
                total_samples += 1;
            }
        }

        let passed = all_differences.is_empty();

        if self.config.verbose {
            self.print_comparison_report(&all_differences, max_difference, total_samples);
        }

        ComparisonResult {
            total_samples,
            max_difference,
            differences: all_differences,
            passed,
        }
    }

    /// Compare two buffers
    fn compare_buffers(
        &self,
        midix: &[f32],
        rusty: &[f32],
        _channel_name: &str,
    ) -> ComparisonResult {
        let mut differences = Vec::new();
        let mut max_difference = 0.0f32;

        for (i, (m, r)) in midix.iter().zip(rusty.iter()).enumerate() {
            let diff = (m - r).abs();
            if diff > self.config.epsilon {
                differences.push(SampleDifference {
                    sample_index: i,
                    midix_value: *m,
                    rusty_value: *r,
                    difference: diff,
                });
            }
            max_difference = max_difference.max(diff);
        }

        let passed = differences.is_empty();

        ComparisonResult {
            total_samples: midix.len(),
            max_difference,
            differences,
            passed,
        }
    }

    /// Print a detailed comparison report
    fn print_comparison_report(
        &self,
        differences: &[SampleDifference],
        max_difference: f32,
        total_samples: usize,
    ) {
        println!("\n=== Comparison Report ===");
        println!("Total samples compared: {total_samples}");
        println!("Maximum difference: {max_difference:.9e}");
        println!(
            "Samples exceeding epsilon ({}): {}",
            self.config.epsilon,
            differences.len()
        );

        if !differences.is_empty() {
            println!(
                "\nFirst {} differences:",
                self.config.max_differences_to_report.min(differences.len())
            );
            for (idx, diff) in differences
                .iter()
                .take(self.config.max_differences_to_report)
                .enumerate()
            {
                println!(
                    "  [{}] Sample {}: midix={:.9}, rusty={:.9}, diff={:.9e}",
                    idx, diff.sample_index, diff.midix_value, diff.rusty_value, diff.difference
                );
            }

            // Find and show largest differences
            let mut sorted_diffs = differences.to_vec();
            sorted_diffs.sort_by(|a, b| b.difference.partial_cmp(&a.difference).unwrap());

            if sorted_diffs.len() > self.config.max_differences_to_report {
                println!(
                    "\nTop {} largest differences:",
                    self.config
                        .max_differences_to_report
                        .min(sorted_diffs.len())
                );
                for (idx, diff) in sorted_diffs
                    .iter()
                    .take(self.config.max_differences_to_report)
                    .enumerate()
                {
                    println!(
                        "  [{}] Sample {}: midix={:.9}, rusty={:.9}, diff={:.9e}",
                        idx, diff.sample_index, diff.midix_value, diff.rusty_value, diff.difference
                    );
                }
            }
        }
    }
}

/// Test scenario builder for common test patterns
pub struct TestScenario {
    pub name: String,
    pub setup: Box<dyn FnMut(&mut SynthesizerComparison)>,
    pub frames_before_action: usize,
    #[allow(clippy::type_complexity)]
    pub action: Option<Box<dyn FnMut(&mut SynthesizerComparison)>>,
    pub frames_after_action: usize,
}

impl TestScenario {
    /// Create a simple note on/off scenario
    pub fn note_on_off(
        channel: u8,
        key: u8,
        velocity: u8,
        frames_before_off: usize,
        frames_after_off: usize,
    ) -> Self {
        Self {
            name: format!("Note on/off - channel:{channel}, key:{key}, velocity:{velocity}",),
            setup: Box::new(move |synth| synth.note_on(channel, key, velocity)),
            frames_before_action: frames_before_off,
            action: Some(Box::new(move |synth| synth.note_off(channel, key))),
            frames_after_action: frames_after_off,
        }
    }

    /// Create a pitch bend scenario
    pub fn pitch_bend(
        channel: u8,
        key: u8,
        velocity: u8,
        bend_value: u16,
        frames_before_bend: usize,
        frames_after_bend: usize,
    ) -> Self {
        Self {
            name: format!("Pitch bend - channel:{channel}, bend:{bend_value}"),
            setup: Box::new(move |synth| synth.note_on(channel, key, velocity)),
            frames_before_action: frames_before_bend,
            action: Some(Box::new(move |synth| synth.pitch_bend(channel, bend_value))),
            frames_after_action: frames_after_bend,
        }
    }

    /// Create a controller change scenario
    pub fn controller_change(
        channel: u8,
        key: u8,
        velocity: u8,
        controller: u8,
        value: u8,
        frames_before: usize,
        frames_after: usize,
    ) -> Self {
        Self {
            name: format!(
                "Controller change - channel:{channel}, controller:{controller}, value:{value}",
            ),
            setup: Box::new(move |synth| synth.note_on(channel, key, velocity)),
            frames_before_action: frames_before,
            action: Some(Box::new(move |synth| {
                synth.controller(channel, controller, value)
            })),
            frames_after_action: frames_after,
        }
    }

    /// Run the test scenario and return the result
    pub fn run(&mut self, synth: &mut SynthesizerComparison) -> ComparisonResult {
        // Reset synthesizers
        synth.reset();

        // Run setup
        (self.setup)(synth);

        // Render frames before action
        let mut all_differences = Vec::new();
        let mut max_difference = 0.0f32;
        let mut total_samples = 0;

        if self.frames_before_action > 0 {
            let result = synth.render_and_compare_frames(self.frames_before_action);
            all_differences.extend(result.differences);
            max_difference = max_difference.max(result.max_difference);
            total_samples += result.total_samples;
        }

        // Run action if present
        if let Some(action) = &mut self.action {
            (action)(synth);
        }

        // Render frames after action
        if self.frames_after_action > 0 {
            let result = synth.render_and_compare_frames(self.frames_after_action);
            all_differences.extend(result.differences);
            max_difference = max_difference.max(result.max_difference);
            total_samples += result.total_samples;
        }

        let passed = all_differences.is_empty();

        if synth.config.verbose {
            println!("\n=== Test Scenario: {} ===", self.name);
            synth.print_comparison_report(&all_differences, max_difference, total_samples);
        }

        ComparisonResult {
            total_samples,
            max_difference,
            differences: all_differences,
            passed,
        }
    }
}
