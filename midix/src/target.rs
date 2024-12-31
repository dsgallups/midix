use crate::message::MidiMessage;
/*
TODO: StreamingMIDITarget vs MIDITarget

StreamingMIDITarget can take in events and play them

MIDITarget takes in a whole file


We should have something that will become  atarget

events are a target? not sure



- we need a pull based event reader from a source
- we need a midi file from a source


- Vec<u8> -> MidiFile (if u8 is smf)
    - will need to subdivide those into [u8]s so they become
    - midi byte messages and stuff
- Vec<u8> -> Vec<MidiByteMessage>
- Vec<u8> -> MidiByteMessage

- [u8] -> MidiByteMessage
- [u8; 3] -> MidiByteMessage
- [u8; 2] -> MidiByteMessage
- [u8; 1] -> MidiByteMessage


- Vec<MidiByteMessage> -> MidiTrack
- Vec<MidiByteMessage> -> Vec<MidiMessage>
- MidiByteMessage -> MidiMessage

- MidiFile -> Vec<u8> //save
- MidiTrack -> MidiFile
- Vec<MidiMessage> -> MidiTrack

- MidiFile -> MidiStream
-

MidiByteMessage vs MidiMessage

MidiFile should have a play which will lead to
StreamingMidiFile.
This will have a tick() method which will
yield a Vec<MidiMessage>

what does MidiStream yield? MidiBytes or MidiMessages? probably MidiMessages
MidiStream does not yield SMFChunks


on streamers:
- MidiDeviceInput is a MidiStreams
- Files can become MidiStreams
    - which means they don't have a tick() function?


or maybe it's a type that's called MidiStreamer
MidiStreamer (file) will be a future that will tick self on polling.
So impl of future for file will be a ticking
function on poll given time elapsed

whereas impl of future for literal input will be
analogous to what's found in midix_piano



a midi stream is a crossbeam Receiver<MidiMessage>
    - MidiMessage has a timestamp, and the message


- u8 -> midi file (if u8 is smf)
- u8 -> midi stream


- we need a midil

- what should Reader do? it should read into.


Streamer should yield MidiMessage
(since caller will know the tick)

whereas static types,
like MidiFile will yield TrackEvent

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
