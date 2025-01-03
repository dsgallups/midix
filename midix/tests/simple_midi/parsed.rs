use midix::{
    events::LiveEvent,
    file::{MidiFile, TimedEvent},
    prelude::{ChannelId, VoiceEvent},
    DataByte, Dynamic, Note, Octave,
};

#[test]
fn test_parse() {
    let parsed = MidiFile::parse(include_bytes!("./simple.mid")).unwrap();

    assert_eq!(parsed.tracks().len(), 1);

    let track = parsed.tracks()[0];

    let mut events = track.events().iter().skip(3);

    note_on(events.next().unwrap(), 0, 3, Note::C, 3, Dynamic::ff());
    note_on(events.next().unwrap(), 0, 3, Note::C, 4, Dynamic::ff());
    note_on(events.next().unwrap(), 96, 2, Note::G, 4, Dynamic::mf());
    note_on(events.next().unwrap(), 192, 1, Note::E, 5, Dynamic::p());
    note_off(events.next().unwrap(), 384, 3, Note::C, 3);
    note_off(events.next().unwrap(), 384, 3, Note::C, 4);
    note_off(events.next().unwrap(), 384, 2, Note::G, 4);
    note_off(events.next().unwrap(), 384, 1, Note::E, 5);
}
fn note_on(
    e: &TimedEvent<LiveEvent<'_>>,
    accumulated_ticks: u32,
    channel_id: u8,
    note: Note,
    octave: i8,
    dynamic: Dynamic,
) {
    assert_eq!(e.accumulated_ticks(), accumulated_ticks);
    let LiveEvent::ChannelVoice(cv) = e.event() else {
        panic!();
    };

    assert_eq!(cv.channel(), ChannelId::new(channel_id).unwrap());
    let VoiceEvent::NoteOn { key, velocity } = cv.event() else {
        panic!();
    };
    assert_eq!(key.note(), note);
    assert_eq!(key.octave(), Octave::new(octave));
    assert_eq!(velocity.dynamic(), dynamic);
}

fn note_off(
    e: &TimedEvent<LiveEvent<'_>>,
    accumulated_ticks: u32,
    channel_id: u8,
    note: Note,
    octave: i8,
) {
    assert_eq!(e.accumulated_ticks(), accumulated_ticks);
    let LiveEvent::ChannelVoice(cv) = e.event() else {
        panic!();
    };

    assert_eq!(cv.channel(), ChannelId::new(channel_id).unwrap());
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
