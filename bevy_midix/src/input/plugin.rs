use super::{MidiInput, MidiInputSettings};
use bevy::prelude::*;

#[doc = r#"
Inserts [`MidiInputSettings`] and [`MidiInputConnection`] as resource

Input system utilizes the [`PreUpdate`] schedule
"#]
#[derive(Clone, Copy, Debug, Default)]
pub struct MidiInputPlugin {
    settings: MidiInputSettings,
}

impl Plugin for MidiInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiInput::new(self.settings));
    }
}
