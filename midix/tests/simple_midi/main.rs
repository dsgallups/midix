use midix::{
    file::{MidiFile, MidiFileRef},
    prelude::*,
};
use pretty_assertions::assert_eq;

#[test]
fn midi_file_ref() {
    let bytes = include_bytes!("./simple.mid");
    let mut reader = Reader::from_byte_slice(bytes);

    let midi = MidiFileRef::read(&mut reader).unwrap();

    let chunks = midi.chunks();

    assert_eq!(chunks.len(), 2);

    let header = chunks.first().unwrap();

    assert!(matches!(header, MidiChunk::Header(_)));

    let track = chunks.get(1).unwrap();

    assert!(matches!(track, MidiChunk::Track(_)));
}
#[test]
fn midi_file_simple() {
    let bytes = include_bytes!("./simple.mid");

    let midi = MidiFile::parse(bytes).unwrap();

    let header = midi.header();

    let time = header.time_signature();
    assert_eq!(time.numerator(), 4);
    assert_eq!(time.denominator(), 4);
    assert_eq!(time.clocks_per_click(), 24);
    //assert_eq!()

    //assert!(matches!(track, MidiChunk::Track(_)));
}
