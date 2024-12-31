use crate::message::MidiMessage;
/*
TODO: StreamingMIDITarget vs MIDITarget

StreamingMIDITarget can take in events and play them

MIDITarget takes in a whole file

*/

#[doc = r#"
Some type that can handles MIDI events
"#]
pub trait MidiTarget {
    /// Error emitted by the target if event fails
    type Error;
    /// Process a message
    fn handle_event(&mut self, event: MidiMessage) -> Result<(), Self::Error>;
}
