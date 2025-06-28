use crate::prelude::ChannelVoiceMessage;
use bevy::prelude::*;
use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    event::{NodeEventList, NodeEventType},
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
        ProcBuffers, ProcInfo, ProcessStatus,
    },
};
use rustysynth::{Synthesizer, SynthesizerSettings};
use std::sync::Arc;

/// Configuration for the MIDI synthesizer node
#[derive(Debug, Clone, Component)]
pub struct MidiSynthNode {
    /// The soundfont data
    pub soundfont: Arc<rustysynth::SoundFont>,
    /// Enable reverb and chorus
    pub enable_reverb_and_chorus: bool,
}

impl MidiSynthNode {
    /// Create a new node with a loaded soundfont and reverb/chorus param
    pub fn new(soundfont: Arc<rustysynth::SoundFont>, enable_reverb_and_chorus: bool) -> Self {
        Self {
            soundfont,
            enable_reverb_and_chorus,
        }
    }
}

impl AudioNode for MidiSynthNode {
    type Configuration = EmptyConfig;

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
        _config: &Self::Configuration,
        cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        MidiSynthProcessor::new(self, cx)
    }
}

/// MIDI synthesizer audio node processor
pub struct MidiSynthProcessor {
    synthesizer: Synthesizer,
}

impl MidiSynthProcessor {
    /// Create a new MIDI synthesizer processor
    pub fn new(config: &MidiSynthNode, cx: ConstructProcessorContext) -> Self {
        let mut settings = SynthesizerSettings::new(cx.stream_info.sample_rate.get() as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb_and_chorus;

        let synthesizer =
            Synthesizer::new(&config.soundfont, &settings).expect("Failed to create synthesizer");

        Self { synthesizer }
    }

    /// Process a MIDI command
    fn process_message(&mut self, command: ChannelVoiceMessage) {
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
                if let Some(message) = boxed.downcast_ref::<ChannelVoiceMessage>() {
                    self.process_message(*message);
                }
            }
        });

        let frames = proc_info.frames;

        // guaranteed to be 2 due to our node's STEREO value.
        let (left, right) = outputs.split_at_mut(1);
        // Render audio from the synthesizer
        self.synthesizer
            .render(&mut left[0][..frames], &mut right[0][..frames]);
        ProcessStatus::outputs_not_silent()
    }
}
