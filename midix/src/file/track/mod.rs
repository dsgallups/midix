mod event;
pub use event::*;
mod message;
pub use message::*;

/*
    todo: Text, Lyrics, Markers, CuePoints, MidiChannel

    Midi file should have like copyrights(),
    etc
*/
/// Defines a track of a midi file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiTrack(Vec<MidiTrackEvent>);
impl MidiTrack {
    pub fn new(events: Vec<MidiTrackEvent>) -> Self {
        Self(events)
    }
}
