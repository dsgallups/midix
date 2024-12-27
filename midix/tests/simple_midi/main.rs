use midix::prelude::*;
#[test]
fn midi_file_ref() {
    let bytes = include_bytes!("./simple.mid");
    let mut reader = Reader::from_byte_slice(bytes);

    let Ok(Event::Header(header)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(header.format_type(), MidiFormatType::SingleMultiChannel);
    assert_eq!(header.timing(), Timing::ticks(&[0, 96]));

    let Ok(Event::Track(track)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track.length(), 59);

    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::Meta(Meta::TimeSignature(time_sig)) = track_event.event() else {
        panic!();
    };
    assert_eq!(time_sig.num(), 4);
    assert_eq!(time_sig.den(), 2); //4/4 timing
    assert_eq!(time_sig.clocks_per_click(), 24);
    assert_eq!(time_sig.notated_32nds_per_24_clocks(), 8);

    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);

    // microseconds per quarter note. In this case, it's 120bpm
    let TrackMessage::Meta(Meta::Tempo(tempo)) = track_event.event() else {
        panic!();
    };
    assert_eq!(tempo.micros_per_quarter_note(), 500000);

    //channel 0 program change to 5
    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::new(1).unwrap());
    let VoiceEventRef::ProgramChange { program } = cv.message() else {
        panic!();
    };
    assert_eq!(*program.byte(), 5);
    /*************/

    //channel 2 program change to 46
    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::new(2).unwrap());
    let VoiceEventRef::ProgramChange { program } = cv.message() else {
        panic!();
    };
    assert_eq!(*program.byte(), 46);
    /*************/

    //channel 3 program change to 70
    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::new(3).unwrap());
    let VoiceEventRef::ProgramChange { program } = cv.message() else {
        panic!();
    };
    assert_eq!(*program.byte(), 70);
    /*************/

    // First key for channel 0
    let Ok(Event::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::new(1).unwrap());
    let VoiceEventRef::NoteOn { key, velocity } = cv.message() else {
        panic!();
    };
    //assert_eq!(**program, 70);

    panic!("next event: {:?}", reader.read_event());

    /*let chunks = midi.chunks();

    assert_eq!(chunks.len(), 2);

    let header = chunks.first().unwrap();

    assert!(matches!(header, MidiChunk::Header(_)));

    let track = chunks.get(1).unwrap();

    assert!(matches!(track, MidiChunk::Track(_)));*/
}
