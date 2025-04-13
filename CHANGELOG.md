# 3.0.0 (April 13, 2025)
Welcome to v3. `midix` introduces significant updates and improvements for MIDI handling and integration!

## Notes
This release includes major refactoring, new features, and breaking changes. It focuses on enhancing MIDI file reading, removing unnecessary `Cow<'_, u8>` usage, and improving the `bevy_midix` examples. Please report any issues encountered.

## Features
- Added a new piano and input example for `bevy_midix`.
- Introduced the ability to insert an SF2 resource and customize synthesizer parameters.
- Enhanced `midix_synth` with better soundfont handling and synthesizer improvements.
- Improved documentation across all crates.
- Added support for additional MIDI formats in `midix` file parsing.

## Breaking Changes
- Refactored `MidiOutputPlugin` and `MidiInput` APIs to streamline connection handling.
- Removed `Cow<'_, u8>` usage in many areas for better performance and simplicity.
- Updated `ChannelId` to a `Channel` enum.

## Chores
- Upgraded to the 2024 edition of Rust.
- Updated to Bevy `0.16.0-rc4`.
- Cleaned up lints and documentation.
- Updated dependencies to their latest versions.
- Improved test coverage for `midix` and `bevy_midix`.

**Full Changelog**: https://github.com/dsgallups/midix/compare/2.0.0...3.0.0

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

**Full Changelog**: https://github.com/dsgallups/midix/compare/1.0...2.0.0
