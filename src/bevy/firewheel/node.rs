use crate::prelude::ChannelVoiceMessage;
use firewheel::node::{AudioNode, AudioNodeInfo, AudioNodeProcessor};
use rustysynth::{Synthesizer, SynthesizerSettings};
use std::{boxed::Box, sync::Arc, vec::Vec};

/// Configuration for the MIDI synthesizer node
#[derive(Debug, Clone)]
pub struct MidiSynthNodeConfig {
    /// The soundfont data
    pub soundfont: Arc<rustysynth::SoundFont>,
    /// Sample rate
    pub sample_rate: f32,
    /// Enable reverb
    pub enable_reverb: bool,
    /// Enable chorus
    pub enable_chorus: bool,
    /// Master volume (0.0 to 1.0)
    pub volume: f32,
}

impl AudioNodeConfig for MidiSynthNodeConfig {
    fn into_node_info(self: Box<Self>) -> AudioNodeInfo {
        AudioNodeInfo {
            num_min_supported_inputs: ChannelCount::ZERO,
            num_max_supported_inputs: ChannelCount::ZERO,
            num_min_supported_outputs: ChannelCount::STEREO,
            num_max_supported_outputs: ChannelCount::STEREO,
            default_channel_config: ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::STEREO,
            },
            equal_num_ins_and_outs: false,
            updates: Default::default(),
            label: Some("MIDI Synthesizer".into()),
        }
    }

    fn build_node(
        self: Box<Self>,
        _sample_rate: f64,
        _max_block_frames: usize,
    ) -> Box<dyn AudioNodeProcessor + Send> {
        Box::new(MidiSynthProcessor::new(*self))
    }
}

/// MIDI synthesizer audio node processor
pub struct MidiSynthProcessor {
    synthesizer: Synthesizer,
    volume: f32,
    sample_rate: f32,
    left_buffer: Vec<f32>,
    right_buffer: Vec<f32>,
}

impl MidiSynthProcessor {
    /// Create a new MIDI synthesizer processor
    pub fn new(config: MidiSynthNodeConfig) -> Self {
        let mut settings = SynthesizerSettings::new(config.sample_rate as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb && config.enable_chorus;

        let synthesizer =
            Synthesizer::new(&config.soundfont, &settings).expect("Failed to create synthesizer");

        // Pre-allocate buffers
        let buffer_size = 512; // Default size, will be resized as needed

        Self {
            synthesizer,
            volume: config.volume,
            sample_rate: config.sample_rate,
            left_buffer: vec![0.0; buffer_size],
            right_buffer: vec![0.0; buffer_size],
        }
    }

    /// Process a MIDI command
    fn process_command(&mut self, command: ChannelVoiceMessage) {
        let channel = (command.status() & 0x0F) as i32;
        let command_type = (command.status() & 0xF0) as i32;
        let data1 = command.data_1_byte() as i32;
        let data2 = command.data_2_byte().unwrap_or(0) as i32;

        self.synthesizer
            .process_midi_message(channel, command_type, data1, data2);
    }
}

impl AudioNode for MidiSynthProcessor {
    fn debug_name(&self) -> &'static str {
        "MidiSynthProcessor"
    }

    fn set_param(&mut self, name: &str, value: f32) {
        match name {
            "volume" => self.volume = value.clamp(0.0, 1.0),
            _ => {}
        }
    }
}

impl AudioNodeProcessor for MidiSynthProcessor {
    fn process(
        &mut self,
        _inputs: &[&[f32]],
        outputs: &mut [&mut [f32]],
        events: NodeEventIter,
        proc_info: ProcInfo,
        _clock_info: &ClockInfo,
        _ctx: &mut FirewheelGraphCtx,
    ) {
        // Process incoming MIDI events
        for event in events {
            if let Some(midi_event) = event.as_any().downcast_ref::<MidiNodeEvent>() {
                self.process_command(midi_event.command);
            }
        }

        let frames = proc_info.frames;

        // Ensure we have stereo output
        if outputs.len() >= 2 {
            // Resize buffers if needed
            if self.left_buffer.len() < frames {
                self.left_buffer.resize(frames, 0.0);
                self.right_buffer.resize(frames, 0.0);
            }

            // Clear buffers
            self.left_buffer[..frames].fill(0.0);
            self.right_buffer[..frames].fill(0.0);

            // Render audio from the synthesizer
            self.synthesizer.render(
                &mut self.left_buffer[..frames],
                &mut self.right_buffer[..frames],
            );

            // Copy to output buffers and apply volume
            let left_out = &mut outputs[0][..frames];
            let right_out = &mut outputs[1][..frames];

            if self.volume == 1.0 {
                left_out.copy_from_slice(&self.left_buffer[..frames]);
                right_out.copy_from_slice(&self.right_buffer[..frames]);
            } else {
                for i in 0..frames {
                    left_out[i] = self.left_buffer[i] * self.volume;
                    right_out[i] = self.right_buffer[i] * self.volume;
                }
            }
        }
    }
}

/// Node event for sending MIDI commands to the synthesizer
#[derive(Debug, Clone)]
pub struct MidiNodeEvent {
    pub command: ChannelVoiceMessage,
}
