use super::{MidiOutput, MidiSettings};
use bevy::prelude::*;

#[doc = r#"
Inserts [`MidiInput`] as a resource.

See [`MidiInputSettings`] for configuration options.
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiOutputPlugin {
    /// The settings to apply to [`MidiInput`] on instantiation.
    pub settings: MidiSettings,
}

impl Plugin for MidiOutputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiOutput::new(self.settings));
    }
}
