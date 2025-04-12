use bevy::{
    color::palettes::{
        css::{GREEN, RED},
        tailwind::{YELLOW_200, YELLOW_300},
    },
    prelude::*,
};
use bevy_midix::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, show_available_ports);
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
                    Escape - Disconnect from current port\n",
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

#[derive(Component)]
struct ConnectStatus;

#[derive(Component)]
struct LastMidiEvent;

fn show_available_ports(
    input: Res<MidiInput>,
    mut instructions: Query<&mut TextSpan, With<AvailablePortsText>>,
) {
    if input.is_changed() {
        let mut instructions = instructions.single_mut().unwrap();
        let mut value = String::new();

        for (i, (name, _)) in input.ports().iter().enumerate() {
            value.push_str(&format!("Port {}: {:?}\n", i, name));
        }

        instructions.0 = value;
    }
}
