# midix

A suite of tools used to read, modify, and manage MIDI-related systems

# Crates

- `midix` - MIDI live/file parsing
- `bevy_midix` - MIDI event resource and handlers

## MIDIx feature roadmap
- `no_std`
- Streamer (midir ext)
- Interfaces between `MidiSource` and `Midi` (some representable MIDI type, like a file, events, etc.)
- MIDI writers
  - Streamer (async timed stream event via midir)
  - MidiFile

## Acknowledgments

## `midix`

This crate is inspired by [`midly`](https://github.com/kovaxis/midix)
and [`quick-xml`](https://github.com/tafia/quick-xml).

If you are in need of a MIDI writer, I highly
recommend using `midly`, as this `midix` does not yet
support file writing.

Thanks to these mainters and contributors for inspiration!

## `bevy_midix`

Forked originally from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Huge thank you for the examples and docs!
