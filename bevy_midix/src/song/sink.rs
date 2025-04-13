use bevy::prelude::*;

use crate::synth::Synth;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, connect_sys);
}

/// Just throw everything in here that needs
/// to be appropriately timed (MidiInput is in the update schedule, idk)
#[derive(Resource)]
pub struct MidiSink {}

fn connect_sys(mut commands: Commands, synth: Option<Res<Synth>>) {
    let Some(synth) = synth else { todo!() };

    // need to make a system that will just constantly tick
}
