use crate::prelude::ChannelVoiceMessage;
use bevy::prelude::*;
use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    event::{NodeEventList, NodeEventType},
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, ProcBuffers,
        ProcInfo, ProcessStatus,
    },
};
use rustysynth::{Synthesizer, SynthesizerSettings};
use std::sync::Arc;

/// MIDI synthesizer node component
#[derive(Component, Clone)]
pub struct MidiSynthNode {
    /// Master volume (0.0 to 1.0)
    pub volume: f32,
}

impl Default for MidiSynthNode {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}

/// Configuration for the MIDI synthesizer node
#[derive(Debug, Clone, Component)]
pub struct MidiSynthNodeConfig {
    /// The soundfont data
    pub soundfont: Arc<rustysynth::SoundFont>,
    /// Sample rate
    pub sample_rate: f32,
    /// Enable reverb
    pub enable_reverb: bool,
    /// Enable chorus
    pub enable_chorus: bool,
}

impl Default for MidiSynthNodeConfig {
    fn default() -> Self {
        Self {
            soundfont: Arc::new(
                rustysynth::SoundFont::new(&mut std::io::Cursor::new(Vec::<u8>::new())).unwrap(),
            ),
            sample_rate: 44100.0,
            enable_reverb: true,
            enable_chorus: true,
        }
    }
}

impl AudioNode for MidiSynthNode {
    type Configuration = MidiSynthNodeConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("MIDI Synthesizer")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::STEREO,
            })
            .uses_events(true)
    }

    fn construct_processor(
        &self,
        config: &Self::Configuration,
        _cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        MidiSynthProcessor::new(config.clone(), self.volume)
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
    pub fn new(config: MidiSynthNodeConfig, volume: f32) -> Self {
        let mut settings = SynthesizerSettings::new(config.sample_rate as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb && config.enable_chorus;

        let synthesizer =
            Synthesizer::new(&config.soundfont, &settings).expect("Failed to create synthesizer");

        // Pre-allocate buffers
        let buffer_size = 512; // Default size, will be resized as needed

        Self {
            synthesizer,
            volume,
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

impl AudioNodeProcessor for MidiSynthProcessor {
    fn process(
        &mut self,
        ProcBuffers { outputs, .. }: ProcBuffers,
        proc_info: &ProcInfo,
        mut events: NodeEventList,
    ) -> ProcessStatus {
        // Process incoming MIDI events
        events.for_each(|event| {
            if let NodeEventType::Custom(boxed) = event {
                if let Some(midi_event) = boxed.downcast_ref::<MidiNodeEvent>() {
                    self.process_command(midi_event.command);
                }
            }
        });

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
            if outputs.len() >= 2 {
                let (left_out, rest) = outputs.split_at_mut(1);
                let left_out = &mut left_out[0][..frames];
                let right_out = &mut rest[0][..frames];

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

        ProcessStatus::outputs_not_silent()
    }
}

/// Node event for sending MIDI commands to the synthesizer
#[derive(Debug, Clone)]
pub struct MidiNodeEvent {
    pub command: ChannelVoiceMessage,
}
