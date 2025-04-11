use bevy::prelude::*;

use crate::{input::MidiInputPlugin, output::MidiOutputPlugin, synth::Synth};

pub struct MidiPlugin {
    output: bool,
    input: bool,
    synth: bool,
}

impl Default for MidiPlugin {
    fn default() -> Self {
        Self {
            output: true,
            input: true,
            synth: true,
        }
    }
}

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        if self.input {
            app.add_plugins(MidiInputPlugin);
        }
        if self.output {
            app.add_plugins(MidiOutputPlugin);
        }

        if self.synth {
            app.init_resource::<Synth>();
        }
    }
}
/*

We should have it such that
user can load sf2 file in asset server

user can, on startup, get a handle to that.

Then say
synth.use_soundfont(handle: Handle<SoundFont>);


*/
