use fnv::FnvHashMap;
use midix::{Key, Velocity, prelude::VoiceEvent};

/// Create a section that can be used for looping
#[derive(Default)]
pub struct SimpleSection {
    beat: FnvHashMap<u64, Vec<VoiceEvent>>,
}
impl SimpleSection {
    /// Get a [`SectionBeat`] to do things
    pub fn beat(&mut self, beat_no: u64) -> SectionBeat<'_> {
        SectionBeat {
            section: self,
            beat_no,
        }
    }
    /// get the list of voice events
    pub fn events(&self) -> &FnvHashMap<u64, Vec<VoiceEvent>> {
        &self.beat
    }
}
/// Configure a beat for a section
pub struct SectionBeat<'a> {
    section: &'a mut SimpleSection,
    beat_no: u64,
}

impl<'a> SectionBeat<'a> {
    /// Add one note to play for the beat
    pub fn play(self, key: Key) -> &'a mut SimpleSection {
        let events = self.section.beat.entry(self.beat_no).or_default();
        events.push(VoiceEvent::note_on(key, Velocity::max()));
        self.section
    }
}
