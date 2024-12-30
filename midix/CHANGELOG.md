# 2.0.0 (December 30, 2024)
Welcome to v2. `midix` has new types that will be used for future MIDI representations and methods!

## Notes
Many changes have been made for `midix`. I may have missed something, or there may be some bug
I haven't tested for. Let me know of any issue, and I'll patch ASAP!

## Features
- Crates are now fully documented, READMEs updated, (most) TODOs dealt with.
- Added new `Reader`, which will parse `SMF` files

## Breaking Changes
- Add many new types for `midix`. Types only have an internal `Cow`s. This may change
in the future.
- Alternative representations of MIDI are available
- Created `LiveEvent`s, `FileEvent`s, and `MidiMessage`s. See the `midix` docs for details.
- Utilize `Key` and `Piano` for `bevy_midix`

## Chores
- Updated `bevy_midix` examples

**Full Changelog**: https://github.com/dsgallups/wasm-tracing/compare/1.0.0...2.0.0
