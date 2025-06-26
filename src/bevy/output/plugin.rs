use super::{MidiOutput, MidiSettings};
use bevy::prelude::*;

#[doc = r#"
Inserts [`MidiOutput`] as a resource.

See [`MidiSettings`] for configuration options.
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiOutputPlugin {
    /// The settings to apply to [`MidiOutput`] on instantiation.
    pub settings: MidiSettings,
}

impl Plugin for MidiOutputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiOutput::new(self.settings));
    }
}
