use std::sync::Arc;

use crate::prelude::*;
use midix::prelude::*;

mod settings;
pub use settings::*;

pub struct Synthesizer {}

impl Synthesizer {
    pub fn new(sound_font: Arc<SoundFont>, settings: SynthesizerSettings) -> Self {
        todo!()
    }
}
