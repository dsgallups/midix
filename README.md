# MIDIx
[<img alt="github" src="https://img.shields.io/badge/github-dsgallups/color-gen?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dsgallups/midix)
[<img alt="crates.io" src="https://img.shields.io/crates/v/midix.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/midix)


A suite of tools used to read, modify, and manage MIDI-related systems

### NOTE: The main branch is in development. Stable versions are on their own branches.


## Overview

`midix` provides users with human readable MIDI structures without invariant states. That is, the midi 1.0 specification has been strongly typed such that programatic commands built with this crate are not invariant.

`midix` provides a parser ([`Reader`](crate::prelude::Reader)) to read events from `.mid` files.
calling [`Reader::read_event`](crate::prelude::Reader::read_event) will yield a [`FileEvent`](crate::prelude::FileEvent).

Additionally, `midix` provides the user with [`LiveEvent::from_bytes`](crate::events::LiveEvent), which will parse events from a live MIDI source.

You may also make your own MIDI representation using the provided structs. A significant portion of
this library lives within the `bevy` feature. See details below on usage with the bevy engine.

## Goal
`midix` is NOT designed to be as fast as possible. It is designed for a user to navigate the MIDI format to read and write to. Instead of working directly with bytes, use language to define what your MIDI is supposed to do.

## Getting Started

MIDI can be interpreted in two main ways: through `LiveEvent`s and regular file `Events`.

### Example
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


## Semantic Versioning and Support
`midix` will adhere to semantic versioning. This means that I've opted to use major versions, even if this crate does not consider itself feature complete (you might get a midix `v29.3.1` someday)


## Bevy Support

Midix has been built with the bevy engine in mind. this feature uses `rustysynth` to play midi sounds under the hood!

### Note
When running the examples, try using `cargo run --example <EXAMPLE_NAME> --features example --release` for the best results!

###  Example
```rust, no_run
use bevy_platform::prelude::*;
use std::time::Duration;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use midix::prelude::*;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            MidiPlugin {
                input: None,
                ..Default::default()
            },
        ))
        .add_systems(Startup, load_sf2)
        .add_systems(Update, scale_me)
        .run();
}
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

struct Scale {
    timer: Timer,
    current_key: Key,
    note_on: bool,
    forward: bool,
    incremented_by: u8,
    max_increment: u8,
}

impl Scale {
    pub fn calculate_next_key(&mut self) {
        if self.forward {
            if self.incremented_by == self.max_increment {
                self.forward = false;
                self.incremented_by -= 1;
                self.current_key -= 1;
            } else {
                self.incremented_by += 1;
                self.current_key += 1;
            }
        } else if self.incremented_by == 0 {
            self.forward = true;
            self.incremented_by += 1;
            self.current_key += 1;
        } else {
            self.incremented_by -= 1;
            self.current_key -= 1;
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(200), TimerMode::Repeating);
        Scale {
            timer,
            current_key: Key::new(Note::C, Octave::new(2)),
            note_on: true,
            forward: true,
            incremented_by: 0,
            max_increment: 11,
        }
    }
}

fn scale_me(synth: Res<Synth>, time: Res<Time>, mut scale: Local<Scale>) {
    // don't do anything until the soundfont has been loaded
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());
    if !scale.timer.just_finished() {
        return;
    }
    if scale.note_on {
        //play note on
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(scale.current_key, Velocity::MAX),
        ));
    } else {
        //turn off the note
        _ = synth.push_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(scale.current_key, Velocity::MAX),
        ));
        scale.calculate_next_key()
    }

    scale.note_on = !scale.note_on;
}
```

See `/examples` for details.


## Acknowledgment

This crate was originally forked from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Please check them out if this crate doesn't suit your needs!


## MIDIx feature roadmap
- `no_std`
- Streamer (midir ext)
- Interfaces between `MidiSource` and `Midi` (some representable MIDI type, like a file, events, etc.)
- MIDI writers
  - Streamer (async timed stream event via midir)
  - MidiFile

## General feature schedule
The SUPPORT.md file denotes the length of time major revisions are supported.

When the major version of the crate is incremented, new features for the previous version(s)
will likely be neglected. If you need a non-breaking feature for an older version before the end
of its maintenence period, please let me know!

## Feature roadmap
- `no_std`
- Streamer (midir ext)
- Interfaces between `MidiSource` and `Midi` (some representable MIDI type, like a file, events, etc.)
- MIDI writers
  - Streamer (async timed stream event via midir)
  - MidiFile

## Acknowledgments
A lot of the documentation is copied directly from
[this documentation](http://www.music.mcgill.ca/~ich/classes/mumt306/StandardMIDIfileformat.html).

This reference states "This document may be freely copied in whole or in part provided the copy contains this Acknowledgement.":
```text
This document was originally distributed in text format by The International MIDI Association.
© Copyright 1999 David Back.
EMail: david@csw2.co.uk
Web: http://www.csw2.co.uk
```

Inspired by/copied from

### `midix`

inspired by [`midly`](https://github.com/kovaxis/midly)
and [`quick-xml`](https://github.com/tafia/quick-xml).

If you are in need of a MIDI writer, I highly
recommend using `midly`, as this `midix` does not yet
support file writing.

Thanks to these mainters and contributors for inspiration!

### `bevy` feature

Forked originally from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Huge thank you for the examples and docs!

### hidden `synth` feature

Forked originally from [rustysynth](https://github.com/sinshu/rustysynth).
