use midix::prelude::*;
#[test]
fn midi_file_ref() {
    let bytes = include_bytes!("./simple.mid");
    let mut reader = Reader::from_byte_slice(bytes);

    let Ok(FileEvent::Header(header)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(header.format_type(), FormatType::SingleMultiChannel);
    assert_eq!(header.timing().ticks_per_quarter_note(), Some(96));

    let Ok(FileEvent::Track(track)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track.len(), 59);

    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::Meta(MetaMessage::TimeSignature(time_sig)) = track_event.event() else {
        panic!();
    };
    assert_eq!(time_sig.num(), 4);
    assert_eq!(time_sig.den(), 2); //4/4 timing
    assert_eq!(time_sig.clocks_per_click(), 24);
    assert_eq!(time_sig.notated_32nds_per_24_clocks(), 8);

    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);

    // microseconds per quarter note. In this case, it's 120bpm
    let TrackMessage::Meta(MetaMessage::Tempo(tempo)) = track_event.event() else {
        panic!();
    };
    assert_eq!(tempo.micros_per_quarter_note(), 500000);

    //channel 0 program change to 5
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), ChannelId::new(1).unwrap());
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(*program.byte(), 5);
    /*************/

    //channel 2 program change to 46
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), ChannelId::new(2).unwrap());
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(*program.byte(), 46);
    /*************/

    //channel 3 program change to 70
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), ChannelId::new(3).unwrap());
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(*program.byte(), 70);
    /*************/

    // First key is for channel 3
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_time(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), ChannelId::new(3).unwrap());
    let VoiceEvent::NoteOn { key, velocity } = cv.event() else {
        panic!();
    };
    assert_eq!(key.note(), Note::C);
    assert_eq!(key.octave(), Octave::new(3));
    assert_eq!(velocity.dynamic(), Dynamic::ff());
}
