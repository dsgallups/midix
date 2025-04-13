#![doc = r#"
Module for the [`MidiPlugin`]
"#]
use bevy::prelude::*;

use crate::{input::MidiInputPlugin, output::MidiOutputPlugin, synth::SynthPlugin};

/// Configure the parts you want to include (input, output, synth).
///
/// By default, the output plugin is disabled.
pub struct MidiPlugin {
    /// Include the output plugin. Disabled by default
    pub output: Option<MidiOutputPlugin>,
    /// Include the input plugin. Enabled by default
    pub input: Option<MidiInputPlugin>,
    /// Include an ingame synth. Enabled by default
    ///
    /// Note: synth is separate from OutputPlugin,
    ///
    /// though, it might be a good idea to intertwine these.
    pub synth: Option<SynthPlugin>,
}

impl Default for MidiPlugin {
    fn default() -> Self {
        Self {
            output: None,
            input: Some(MidiInputPlugin::default()),
            synth: Some(SynthPlugin::default()),
        }
    }
}

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        if let Some(input) = self.input {
            app.add_plugins(input);
        }
        if let Some(output) = self.output {
            app.add_plugins(output);
        }

        if let Some(synth_plugin) = self.synth {
            app.add_plugins(synth_plugin);
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
