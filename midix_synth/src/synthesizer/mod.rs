use std::sync::Arc;

use crate::prelude::*;
use midix::prelude::*;

mod settings;
pub use settings::*;

pub struct Synthesizer {
    sound_font: Arc<SoundFont>,
}

impl Synthesizer {
    pub fn new(sound_font: Arc<SoundFont>, settings: SynthesizerSettings) -> Self {
        Self { sound_font }
    }

    pub fn process_voice_message(&mut self, message: ChannelVoiceMessage) {
        let ChannelVoiceMessage { channel, event } = message;

        match event {
            VoiceEvent::NoteOn { key, velocity } => {
                //todo
                todo!()
            }
            VoiceEvent::NoteOff { key, velocity } => {
                //todo
                todo!()
            }
            _ => todo!(),
        }
    }
}
