# MIDIx central repository
[<img alt="github" src="https://img.shields.io/badge/github-dsgallups/color-gen?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dsgallups/midix)

A suite of tools used to read, modify, and manage MIDI-related systems

- `midix`

[<img alt="crates.io" src="https://img.shields.io/crates/v/midix.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/midix)

MIDI live/file parsing/type for programmable MIDI. See [README](https://github.com/dsgallups/midix/blob/main/bevy_midix/README.md) for examples and details!


- `bevy_midix`

[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_midix.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bevy_midix)

Human readable MIDI. See [README](https://github.com/dsgallups/midix/blob/main/midix/README.md) for examples and details!

- `midix_synth`

Streaming midi-command SoundFont synth (Under heavy development). If you're looking for a rust synthesizer with audiofont support, I highly recommend checking out [rustysynth](https://github.com/sinshu/rustysynth) for your needs!


## MIDIx feature roadmap
- `no_std`
- Streamer (midir ext)
- Interfaces between `MidiSource` and `Midi` (some representable MIDI type, like a file, events, etc.)
- MIDI writers
  - Streamer (async timed stream event via midir)
  - MidiFile

## Acknowledgments

### `midix`

This crate is inspired by [`midly`](https://github.com/kovaxis/midly)
and [`quick-xml`](https://github.com/tafia/quick-xml).

If you are in need of a MIDI writer, I highly
recommend using `midly`, as this `midix` does not yet
support file writing.

Thanks to these mainters and contributors for inspiration!

### `bevy_midix`

Forked originally from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Huge thank you for the examples and docs!

### `midix_synth`

Forked originally from [rustysynth](https://github.com/sinshu/rustysynth).
