//! Simple example demonstrating MIDI synthesis using the firewheel backend

use bevy::prelude::*;
use midix::bevy::firewheel::prelude::*;
use midix::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add bevy_seedling's audio plugin
            bevy_seedling::SeedlingPlugin::default(),
            // Add our MIDI plugin
            FirewheelMidiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (play_scale, keyboard_input, volume_control))
        .run();
}

/// Set up the MIDI synthesizer
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a soundfont file
    let soundfont = asset_server.load("soundfont.sf2");

    // Spawn a MIDI synthesizer entity
    commands
        .spawn_midi_synth(soundfont)
        .insert(Name::new("Main Synthesizer"));

    // Spawn a camera
    commands.spawn(Camera2d);

    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "Firewheel MIDI Example\n\
            Press A-K keys to play notes\n\
            Press Space to play a scale\n\
            Press +/- to adjust volume\n\
            Press Escape to stop all notes",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

/// Play a C major scale when space is pressed
fn play_scale(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MidiCommands>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut note_index: Local<usize>,
    mut playing: Local<bool>,
) {
    // Toggle scale playback with space
    if keyboard.just_pressed(KeyCode::Space) {
        *playing = !*playing;
        *note_index = 0;

        // Stop any playing notes when toggling off
        if !*playing {
            for mut commands in &mut query {
                for i in 0..127 {
                    commands.send(ChannelVoiceMessage::note_off(0, i, 0));
                }
            }
        }
    }

    if !*playing {
        return;
    }

    // Initialize timer if needed
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(0.3, TimerMode::Repeating));
    timer.tick(time.delta());

    if timer.just_finished() {
        let scale = [60, 62, 64, 65, 67, 69, 71, 72]; // C major scale

        for mut commands in &mut query {
            // Turn off previous note
            if *note_index > 0 {
                let prev_note = scale[(*note_index - 1) % scale.len()];
                commands.send(ChannelVoiceMessage::note_off(0, prev_note, 0));
            }

            // Play current note
            let note = scale[*note_index % scale.len()];
            commands.send(ChannelVoiceMessage::note_on(0, note, 80));

            *note_index += 1;

            // Stop after playing the scale twice
            if *note_index >= scale.len() * 2 {
                *playing = false;
                commands.send(ChannelVoiceMessage::note_off(
                    0,
                    scale[(*note_index - 1) % scale.len()],
                    0,
                ));
            }
        }
    }
}

/// Handle keyboard input for playing notes
fn keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MidiCommands>) {
    // Map keyboard keys to MIDI notes (C4 to B4)
    let key_to_note = [
        (KeyCode::KeyA, 60), // C4
        (KeyCode::KeyW, 61), // C#4
        (KeyCode::KeyS, 62), // D4
        (KeyCode::KeyE, 63), // D#4
        (KeyCode::KeyD, 64), // E4
        (KeyCode::KeyF, 65), // F4
        (KeyCode::KeyT, 66), // F#4
        (KeyCode::KeyG, 67), // G4
        (KeyCode::KeyY, 68), // G#4
        (KeyCode::KeyH, 69), // A4
        (KeyCode::KeyU, 70), // A#4
        (KeyCode::KeyJ, 71), // B4
        (KeyCode::KeyK, 72), // C5
    ];

    for mut commands in &mut query {
        // Handle note on/off for each key
        for (key, note) in key_to_note {
            if keyboard.just_pressed(key) {
                commands.send(ChannelVoiceMessage::new(
                    Channel::One,
                    VoiceEvent::note_on(
                        Key::from_databyte(note).unwrap(),
                        Velocity::new_unchecked(0),
                    ),
                ));
            }
            if keyboard.just_released(key) {
                commands.send(ChannelVoiceMessage::new(
                    Channel::One,
                    VoiceEvent::note_off(
                        Key::from_databyte(i).unwrap(),
                        Velocity::new_unchecked(0),
                    ),
                ));
            }
        }

        // Panic button - stop all notes
        if keyboard.just_pressed(KeyCode::Escape) {
            for i in 0..127 {
                commands.send(ChannelVoiceMessage::new(
                    Channel::One,
                    VoiceEvent::note_off(
                        Key::from_databyte(i).unwrap(),
                        Velocity::new_unchecked(0),
                    ),
                ));
            }
        }
    }
}

/// Handle volume control
fn volume_control(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MidiSynthConfig>) {
    for mut config in &mut query {
        if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
            config.volume = (config.volume + 0.1).min(1.0);
            info!("Volume: {:.1}", config.volume);
        }
        if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
            config.volume = (config.volume - 0.1).max(0.0);
            info!("Volume: {:.1}", config.volume);
        }
    }
}
