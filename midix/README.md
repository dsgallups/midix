Human readable MIDI structures



# Overview

`midix` provides a parser ([`Reader`](crate::prelude::Reader)) to read events from `.mid` files.
calling [`Reader::read_event`](crate::prelude::Reader::read_event) will yield a [`FileEvent`](crate::prelude::FileEvent).

Additionally, `midix` provides the user with [`LiveEvent::from_bytes`](crate::events::LiveEvent), which will parse events from a live MIDI source.

You may also make your own MIDI representation using the provided structs. If check out
[`bevy_midix`](https://github.com/dsgallups/midix/bevy_midix) if you'd like to use `midix` in your games!

## Goal
`midix` is NOT designed to be as fast as possible. It is designed for a user to navigate the MIDI format to read and write to. Instead of working directly with bytes, use language to define what your MIDI is supposed to do.

# Getting Started

MIDI can be interpreted in two main ways: through `LiveEvent`s and regular file `Events`.

# Example
To read from a file, use the [`Reader`](crate::prelude::Reader):
```rust
use midix::prelude::*;

let midi_header = [
    /* MIDI Header */
    0x4D, 0x54, 0x68, 0x64, // "MThd"
    0x00, 0x00, 0x00, 0x06, // Chunk length (6)
    0x00, 0x00, // format 0
    0x00, 0x01, // one track
    0x00, 0x60  // 96 per quarter note
];

let mut reader = Reader::from_byte_slice(&midi_header);

// The first and only event will be the midi header
let Ok(FileEvent::Header(header)) = reader.read_event() else {
    panic!("Expected a header event");
};

// format 0 implies a single multi-channel file (only one track)
assert_eq!(header.format_type(), FormatType::SingleMultiChannel);

assert_eq!(
    header.timing().ticks_per_quarter_note(),
    Some(96)
);

```
To parse a [`LiveEvent`](crate::prelude::LiveEvent)

```rust
use midix::prelude::*;

/* Ch.3 Note On C4, forte */
let note_on = [0x92, 0x3C, 0x60];

// NoteOn is a channel voice message
// Alternatively, use VoiceEvent::read_bytes(&note_on)
let Ok(LiveEvent::ChannelVoice(channel_voice_msg)) = LiveEvent::from_bytes(&note_on) else {
    panic!("Expected a channel voice event");
};

let VoiceEvent::NoteOn { key, velocity } = channel_voice_msg.event() else {
    panic!("Expected a note on event");
};

assert_eq!(channel_voice_msg.channel(), Channel::Three);
assert_eq!(key.note(), Note::C);
assert_eq!(key.octave(), Octave::new(4));
assert_eq!(velocity.byte(), 96);
```


# Semantic Versioning and Support
`midix` will adhere to semantic versioning.

## General feature schedule
The SUPPORT.md file denotes the length of time major revisions are supported.

When the major version of the crate is incremented, new features for the previous version(s)
will likely be neglected. If you need a feature for an older version before the end
of its maintenence period, please let me know!

# Feature roadmap
- `no_std`
- Streamer (midir ext)
- Interfaces between `MidiSource` and `Midi` (some representable MIDI type, like a file, events, etc.)
- MIDI writers
  - Streamer (async timed stream event via midir)
  - MidiFile

# Acknowledgments
A lot of the documentation is copied directly from
[this documentation](http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html).

This reference states "This document may be freely copied in whole or in part provided the copy contains this Acknowledgement.":
```text
This document was originally distributed in text format by The International MIDI Association.
© Copyright 1999 David Back.
EMail: david@csw2.co.uk
Web: http://www.csw2.co.uk
```
