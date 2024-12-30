# bevy_MIDIx
Bevy plugin that uses [`midix`](https://crates.io/crates/midix),
[`midir`](https://github.com/Boddlnagg/midir), and [`crossbeam`](https://github.com/crossbeam-rs/crossbeam).

Read from and write to MIDI devices!

# Example
```rust, no_run
use bevy::prelude::*;
use bevy_midix::prelude::*;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(MidiInputPlugin)
    .add_plugins(MidiOutputPlugin)
    .run();
```

See `/examples` for details.


# Acknowledgment

This crate HEAVILY borrows its documentation and types
from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Please
check them out if this crate doesn't suit your needs!
