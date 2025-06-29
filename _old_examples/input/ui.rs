use bevy::{
    color::palettes::{
        css::{GREEN, RED},
        tailwind::{YELLOW_200, YELLOW_300},
    },
    prelude::*,
};
use midix::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (update_available_ports, update_connection_status));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            Text::default(),
            TextFont {
                font: font.clone(),
                font_size: 30.,
                ..default()
            },
        ))
        .with_children(|commands| {
            commands.spawn((
                TextSpan::new(
                    "INSTRUCTIONS\n\
                    R - Refresh ports\n\
                    0 to 9 - Connect to port\n\
                    Escape - Disconnect from current port\n\n",
                ),
                TextFont {
                    font: font.clone(),
                    font_size: 30.,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
            commands.spawn((
                TextSpan::new("Available Ports: \n"),
                TextFont {
                    font: font.clone(),
                    font_size: 30.,
                    ..Default::default()
                },
                TextColor(YELLOW_200.into()),
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(GREEN.into()),
                AvailablePortsText,
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(RED.into()),
                ConnectStatus,
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(YELLOW_300.into()),
                LastMidiEvent,
            ));
        });
}

#[derive(Component)]
struct AvailablePortsText;

fn update_available_ports(
    input: Res<MidiInput>,
    mut instructions: Query<&mut TextSpan, With<AvailablePortsText>>,
) {
    if input.is_changed() {
        let mut instructions = instructions.single_mut().unwrap();
        let mut value = String::new();

        for (i, port) in input.ports().iter().enumerate() {
            value.push_str(&format!("Port {}: {:?}\n", i, port.id()));
        }
        value.push('\n');

        instructions.0 = value;
    }
}

#[derive(Component)]
struct ConnectStatus;

// this should probably be part of Res<MidiInput>
//
// and may want to be able to accept input from many midi devices
fn update_connection_status(
    connections: Res<MidiInput>,
    mut status: Query<(&mut TextSpan, &mut TextColor), With<ConnectStatus>>,
) {
    let (mut status, mut color) = status.single_mut().unwrap();
    if connections.is_active() {
        status.0 = "Connected".to_string();
        color.0 = GREEN.into();
    } else {
        status.0 = "Disconnected".to_string();
        color.0 = RED.into();
    }
}

#[derive(Component)]
struct LastMidiEvent;
