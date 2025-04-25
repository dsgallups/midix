#![doc = r#"
Module for the [`MidiPlugin`]
"#]
use bevy::prelude::*;

use crate::{
    MidiSettings,
    asset::{MidiFile, MidiFileLoader},
    input::MidiInputPlugin,
    output::MidiOutputPlugin,
    synth::{SynthParams, SynthPlugin},
};

/// Configure the parts you want to include (input, output, synth).
///
/// By default, the output plugin is disabled.
pub struct MidiPlugin {
    /// Include the output plugin. Disabled by default
    pub output: Option<MidiOutputPlugin>,
    /// Include the input plugin. Enabled by default
    pub input: Option<MidiSettings>,
    /// Include an ingame synth. Enabled by default
    ///
    /// Note: synth is separate from OutputPlugin,
    ///
    /// though, it might be a good idea to intertwine these.
    pub synth: Option<SynthParams>,
}

impl Default for MidiPlugin {
    fn default() -> Self {
        Self {
            output: None,
            input: Some(MidiSettings::default()),
            synth: Some(SynthParams::default()),
        }
    }
}

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<MidiFileLoader>()
            .init_asset::<MidiFile>();
        if let Some(settings) = self.input {
            app.add_plugins(MidiInputPlugin { settings });
        }
        if let Some(output) = self.output {
            app.add_plugins(output);
        }

        if let Some(synth_plugin) = self.synth {
            app.add_plugins(SynthPlugin {
                params: synth_plugin,
            });
        }
    }
}
