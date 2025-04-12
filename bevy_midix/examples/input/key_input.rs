use bevy::prelude::*;
use bevy_midix::prelude::*;

/// TODO: experimenting in here
pub fn plugin(app: &mut App) {
    app.add_systems(Update, (refresh_inputs, connect, disconnect));
}

fn refresh_inputs(keys: Res<ButtonInput<KeyCode>>, mut port_event: EventWriter<MidiInputEvent>) {
    if keys.just_pressed(KeyCode::KeyR) {
        port_event.write(MidiInputEvent::RefreshPorts);
    }
}

fn connect(
    keys: Res<ButtonInput<KeyCode>>,
    input: Res<MidiInputPorts>,
    connections: Query<&MidiInputConnection>,
    mut port_event: EventWriter<MidiInputEvent>,
) {
    if !connections.is_empty() {
        return;
    }
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
            if let Some(port) = input.ports().get(index) {
                debug!("Connecting to {}", port.id());
                port_event.write(MidiInputEvent::ConnectToPort(port.id()));
            }
        }
    }
}
fn disconnect(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    connections: Query<(Entity, &mut MidiInputConnection)>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    for (entity, mut connection) in connections {
        connection.close();
        commands.entity(entity).despawn();
    }
}
