use bevy::prelude::*;
use midix::prelude::*;
/// TODO: experimenting in here
pub fn plugin(app: &mut App) {
    app.add_systems(Update, (refresh_inputs, connect, disconnect));
}

fn refresh_inputs(keys: Res<ButtonInput<KeyCode>>, mut midi_input: ResMut<MidiInput>) {
    if keys.just_pressed(KeyCode::KeyR) {
        midi_input.refresh_ports();
    }
}

fn connect(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<MidiInput>) {
    for (keycode, index) in [
        (KeyCode::Digit0, 0),
        (KeyCode::Digit1, 1),
        (KeyCode::Digit2, 2),
        (KeyCode::Digit3, 3),
        (KeyCode::Digit4, 4),
        (KeyCode::Digit5, 5),
        (KeyCode::Digit6, 6),
        (KeyCode::Digit7, 7),
        (KeyCode::Digit8, 8),
        (KeyCode::Digit9, 9),
    ] {
        if keys.just_pressed(keycode) {
            debug!("Connecting to {}", index);
            match input.connect_to_index(index) {
                Ok(()) => {
                    debug!("Connected!");
                }
                Err(e) => {
                    debug!("Couldn't connect: {e:?}");
                }
            }
        }
    }
}
fn disconnect(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<MidiInput>) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    input.disconnect();
}
