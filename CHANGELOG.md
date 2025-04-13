# 3.1.0 (April 13, 2025)
## `bevy_midix`
- Added `iterate_voices` example
- Added `soundfont.sf2` to assets

## `midix`
- `Key::new` is now const
- `Octave::new` is now const

## `midix_synth`
- if the sanity check fails (soundfont data is readable but appears invalid), it will warn, not error.


# 3.0.0 (April 13, 2025)
Welcome to v3. `midix` introduces significant updates and improvements for MIDI handling and integration!

## Notes
This release includes major refactoring, new features, and breaking changes. It focuses on enhancing MIDI file reading, removing unnecessary `Cow<'_, u8>` usage, and a complete refactor of `bevy_midix`. Please report any issues encountered.

## `bevy_midix`
### Features
- Added a new piano and input example.
  - In the future, I will add the usage of a MIDI file.
- Introduced the ability to insert an SF2 resource and customize synthesizer parameters (sample rate).

See examples for details.

- Enhanced `midix_synth` with better soundfont handling and synthesizer improvements.
- Added support for additional MIDI formats in `midix` file parsing.

### Breaking Changes
- Refactored `MidiOutput` and `MidiInput` APIs to streamline connection handling.

Note, especially for linux users:

I have unsafely implemented `Sync` for these resources. This is because `midir` uses `alsa` for midi IO under the hood for this target. I have **assumed** that users will NOT have a second
`midir::MidiInput` or `midir::MidiOutput` instance in their programs. I have not run into issues testing on my
own linux device, but this implementation *MAY* be frivolous. If you find `UB`, please let me know ASAP so I can
work on finding an alternative. I expect UB to represent itself as these plugins not listening nor active,
so errors should only be confined to safely handled interfaces.

- for `MidiOutput`, you may send only types that implement `Into<MidiMessageBytes>`.

Implementors of this type should be all `LiveEvent`s, but there may be others that
will be included in futures minor releases.



### Chores
- Updated to Bevy `0.16.0-rc4`.
  - Will update to `0.16` for `v3.1.0`.

## `midix`

### Features
- `.mid` file reading is now possible! see `/midix/tests/read_all.rs` for a reference.

### Breaking Changes
- Removed `Cow<'_, u8>` usage in many areas for better performance and simplicity.
It is faster to copy a `u8` than it is to reference the `u8`. Starting with this was more
of an experiment than a decent way to be "min-copy". As a result, there is an opportunity now
to create a parsing Struct that references an underlying byte slice, like that of [ttf-parser's Face](https://docs.rs/ttf-parser/0.25.1/ttf_parser/struct.Face.html).

- Updated `ChannelId` to a `Channel` enum.
The `Channel` enum is now an enumeration of "One" to "Sixteen".

### Notes

- Reading/Writing to byte slices
Some of the APIs need work, particularly for reading to and writing to byte slices.

I will be cleaning this up for v4, and will begin deprecating poor interfaces throughout
v3 minor releases.

This work will include progress to writing out to midi slices

- ControlChange

Currently, there are things like pedals, external devices
that deserve an enumeration rather than a raw byte value. I will be working
on this conversion throughout v3, and hopefully as the default type for v4.




## Chores
- Upgraded all crates to the 2024 edition of Rust.
- Improved documentation across all crates.
- Updated dependencies to their latest versions.
- Improved test coverage for `midix` and `bevy_midix`.

## Final Notes
There are some methods that now return other types.
If I have missed something important in these release notes,
do not hesistate to file an issue explaining your frustrations.

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
