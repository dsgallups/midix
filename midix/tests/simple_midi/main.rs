use midix::{file::MidiFile, prelude::*};
use pretty_assertions::assert_eq;

#[test]
fn midi_file() {
    let bytes = include_bytes!("./simple.mid");
    let mut reader = Reader::from_byte_slice(bytes);

    let midi = MidiFile::read(&mut reader).unwrap();

    let mut iter = bytes.iter();
    let mut v = iter.next();

    let chunks = midi.chunks();

    assert_eq!(chunks.len(), 2);

    let header = chunks.first().unwrap();

    assert!(matches!(header, MidiChunk::Header(_)));

    let track = chunks.get(1).unwrap();

    assert!(matches!(track, MidiChunk::Track(_)));
}
