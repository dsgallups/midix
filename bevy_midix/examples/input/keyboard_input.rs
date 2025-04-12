use bevy::prelude::*;
use bevy_midix::prelude::*;

/// TODO: experimenting in here
pub fn plugin(app: &mut App) {
    app.add_systems(Update, (refresh_inputs, connect));
}

fn refresh_inputs(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
    if keys.just_pressed(KeyCode::KeyR) {
        input.refresh_ports();
    }
}

fn connect(keys: Res<ButtonInput<KeyCode>>, input: Res<MidiInput>) {
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
            if let Some((name, port)) = input.ports().get(index) {
                input.connect(port.clone());
                debug!("Connecting to {name}");
            }
        }
    }
}
