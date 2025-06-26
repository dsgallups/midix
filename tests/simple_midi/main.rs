use midix::prelude::*;
mod parsed;

/*
4D546864            - "MThd"
00000006            - chunk length: next 6 bytes
00000001            - | 00 00: format 0 | 00 01: one track |
0060                - 00 60: 96/qn
4D54726B            - "MTrk"
0000003B            - chunk length: next 59 bytes

Note the following table:
Midi Event            | DeltaT dec   |
                      | w [Acc]      | Event and Details
---------------------------------------------------------------------------------------------------
00 FF5804 04021808  - | 00 [00]      | Meta Time Signature
                      |              | (4 bytes; 4/4 time; 24 MIDI clocks/click
                      |              | 8 32nd notes/24 MIDI clocks (24 MIDI clocks = 1 beat)
                      |              |
00 FF5103 07A120    - |  00 [00]     | Track Event Tempo
                      |              | (3 bytes, 500_000 microsecs / quarter note = 120 beats/min)
                      |              |
00 C005             - |  00 [00]     | Ch1 Program Change to 5 (Electric Piano 2)
                      |              |
00 C12E             - |  00 [00]     | Ch2 Program Change to 46 (Harp)
                      |              |
00 C246             - |  00 [00]     | Ch3 Program Change to 71 (Bassoon)
                      |              |
00 923060           - |  00 [00]     | Ch3 Note on C3, Forte (96)
                      |              |
00 3C60             - |  00 [00]     | Note: Running Status
                      |              | Ch3 Note on C4, Forte (96)
                      |              |
60 914340           - |  96 [96]     | Ch2 Note on G4, Mezzo-Forte (64)
                      |              |
60 904C20           - |  96 [192]    | Ch1 Note on E5, Piano (32)
                      |              |
8140 823040         - | 192 [384]    | Note: Two-byte Delta time
                      |              | Ch3 Note off C3
                      |              |
00 3C40             - |  00 [384]    | Note: Running Status
                      |              | Ch3 Note off
                      |              |
00 814340           - |  00 [384]    | Ch2 Note off G4
                      |              |
00 804C4            - |  00 [384]    | Ch1 Note off E5
                      |              |
00 0FF2F00          - |  00 [384]    | end of track
*/

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
    assert_eq!(program.byte(), 5);
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
    assert_eq!(program.byte(), 46);
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
    assert_eq!(program.byte(), 70);
    /*************/

    use Channel::*;
    note_on(&mut reader, 0, Three, Note::C, 3, Dynamic::ff());
    note_on(&mut reader, 0, Three, Note::C, 4, Dynamic::ff());
    note_on(&mut reader, 96, Two, Note::G, 4, Dynamic::mf());
    note_on(&mut reader, 96, One, Note::E, 5, Dynamic::p());
    note_off(&mut reader, 192, Three, Note::C, 3);
    note_off(&mut reader, 0, Three, Note::C, 4);
    note_off(&mut reader, 0, Two, Note::G, 4);
    note_off(&mut reader, 0, One, Note::E, 5);
}

fn note_on(
    reader: &mut Reader<&[u8]>,
    delta_ticks: u32,
    channel: Channel,
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
    assert_eq!(cv.channel(), channel);
    let VoiceEvent::NoteOn { key, velocity } = cv.event() else {
        panic!();
    };
    assert_eq!(key.note(), note);
    assert_eq!(key.octave(), Octave::new(octave));
    assert_eq!(velocity.dynamic(), dynamic);
}

fn note_off(
    reader: &mut Reader<&[u8]>,
    delta_ticks: u32,
    channel: Channel,
    note: Note,
    octave: i8,
) {
    let Ok(FileEvent::TrackEvent(track_event)) = reader.read_event() else {
        panic!()
    };
    assert_eq!(track_event.delta_ticks(), delta_ticks);
    let TrackMessage::ChannelVoice(cv) = track_event.event() else {
        panic!();
    };
    assert_eq!(cv.channel(), channel);

    match cv.event() {
        VoiceEvent::NoteOn { key, velocity } => {
            assert_eq!(velocity.byte(), 0);
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
