use midix::prelude::*;
use num_enum::TryFromPrimitive;
mod parsed;
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
    assert_eq!(track_event.delta_ticks(), 0);
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
    assert_eq!(track_event.delta_ticks(), 0);

    // microseconds per quarter note. In this case, it's 120bpm
    let TrackMessage::Meta(MetaMessage::Tempo(tempo)) = track_event.event() else {
        panic!();
    };
    assert_eq!(tempo.micros_per_quarter_note(), 500000);

    //channel 0 program change to 5
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::One);
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(program.byte().value(), 5);
    /*************/

    //channel 2 program change to 46
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::Two);
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(program.byte().value(), 46);
    /*************/

    //channel 3 program change to 70
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), 0);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), Channel::Three);
    let VoiceEvent::ProgramChange { program } = cv.event() else {
        panic!();
    };
    assert_eq!(program.byte().value(), 70);
    /*************/

    note_on(&mut reader, 0, 3, Note::C, 3, Dynamic::ff());
    note_on(&mut reader, 0, 3, Note::C, 4, Dynamic::ff());
    note_on(&mut reader, 96, 2, Note::G, 4, Dynamic::mf());
    note_on(&mut reader, 96, 1, Note::E, 5, Dynamic::p());
    note_off(&mut reader, 192, 3, Note::C, 3);
    note_off(&mut reader, 0, 3, Note::C, 4);
    note_off(&mut reader, 0, 2, Note::G, 4);
    note_off(&mut reader, 0, 1, Note::E, 5);
}

fn note_on(
    reader: &mut Reader<&[u8]>,
    delta_ticks: u32,
    channel_id: u8,
    note: Note,
    octave: i8,
    dynamic: Dynamic,
) {
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), delta_ticks);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(
        cv.channel(),
        Channel::try_from_primitive(channel_id).unwrap()
    );
    let VoiceEvent::NoteOn { key, velocity } = cv.event() else {
        panic!();
    };
    assert_eq!(key.note(), note);
    assert_eq!(key.octave(), Octave::new(octave));
    assert_eq!(velocity.dynamic(), dynamic);
}

fn note_off(reader: &mut Reader<&[u8]>, delta_ticks: u32, channel_id: u8, note: Note, octave: i8) {
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), delta_ticks);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(
        cv.channel(),
        Channel::try_from_primitive(channel_id).unwrap()
    );

    match cv.event() {
        VoiceEvent::NoteOn { key, velocity } => {
            assert_eq!(velocity.byte(), DataByte::new_unchecked(0));
            assert_eq!(key.note(), note);
            assert_eq!(key.octave(), Octave::new(octave));
        }
        VoiceEvent::NoteOff { key, velocity: _ } => {
            assert_eq!(key.note(), note);
            assert_eq!(key.octave(), Octave::new(octave));
        }
        _ => panic!(),
    }
}
