use crate::bevy::asset::SoundFont;
use crate::bevy::firewheel::components::*;
use crate::prelude::*;
use bevy::prelude::*;

/// System for note input handling
pub fn handle_note_input(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MidiCommands>) {
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
    ];

    for mut commands in &mut query {
        for (key, note) in key_to_note {
            if keyboard.just_pressed(key) {
                // Send note on
                if let Ok(midi_key) = Key::from_databyte(note) {
                    commands.send(ChannelVoiceMessage::new(
                        Channel::One,
                        VoiceEvent::note_on(midi_key, Velocity::new_unchecked(100)),
                    ));
                }
            }
            if keyboard.just_released(key) {
                // Send note off
                if let Ok(midi_key) = Key::from_databyte(note) {
                    commands.send(ChannelVoiceMessage::new(
                        Channel::One,
                        VoiceEvent::note_off(midi_key, Velocity::ZERO),
                    ));
                }
            }
        }
    }
}

/// System for playing a simple scale
pub fn play_scale(
    mut query: Query<&mut MidiCommands>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut note_index: Local<usize>,
) {
    // Initialize timer on first run
    if timer.duration().as_secs_f32() == 0.0 {
        *timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        let scale = [60, 62, 64, 65, 67, 69, 71, 72]; // C major scale

        for mut commands in &mut query {
            // Turn off previous note
            if *note_index > 0 {
                let prev_note = scale[(*note_index - 1) % scale.len()];
                if let Ok(key) = Key::from_databyte(prev_note) {
                    commands.send(ChannelVoiceMessage::new(
                        Channel::One,
                        VoiceEvent::note_off(key, Velocity::ZERO),
                    ));
                }
            }

            // Play current note
            let note = scale[*note_index % scale.len()];
            if let Ok(key) = Key::from_databyte(note) {
                commands.send(ChannelVoiceMessage::new(
                    Channel::One,
                    VoiceEvent::note_on(key, Velocity::new_unchecked(80)),
                ));
            }

            *note_index += 1;
        }
    }
}

/// System for sending program change (instrument selection)
pub fn set_instrument(
    mut query: Query<(&mut MidiCommands, &MidiInstrument), Changed<MidiInstrument>>,
) {
    for (mut commands, instrument) in &mut query {
        // Map channel index to Channel enum
        let channel = match instrument.channel {
            0 => Channel::One,
            1 => Channel::Two,
            2 => Channel::Three,
            3 => Channel::Four,
            4 => Channel::Five,
            5 => Channel::Six,
            6 => Channel::Seven,
            7 => Channel::Eight,
            8 => Channel::Nine,
            9 => Channel::Ten,
            10 => Channel::Eleven,
            11 => Channel::Twelve,
            12 => Channel::Thirteen,
            13 => Channel::Fourteen,
            14 => Channel::Fifteen,
            15 => Channel::Sixteen,
            _ => Channel::One, // Default to channel 1 for invalid values
        };

        if let Ok(program) = Program::new(instrument.program) {
            commands.send(ChannelVoiceMessage::new(
                channel,
                VoiceEvent::program_change(program),
            ));
        }
    }
}

/// Component for specifying MIDI instrument
#[derive(Component)]
pub struct MidiInstrument {
    pub channel: u8,
    pub program: u8,
}

impl Default for MidiInstrument {
    fn default() -> Self {
        Self {
            channel: 0,
            program: 0, // Acoustic Grand Piano
        }
    }
}

/// System for panic button - stops all notes
pub fn panic_button(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MidiCommands>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        for mut commands in &mut query {
            // Send note off for all notes on all channels
            for channel in Channel::all() {
                for note_num in 0..128 {
                    if let Ok(key) = Key::from_databyte(note_num) {
                        commands.send(ChannelVoiceMessage::new(
                            channel,
                            VoiceEvent::note_off(key, Velocity::ZERO),
                        ));
                    }
                }
            }
        }
    }
}

/// System for volume control
pub fn volume_control(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MidiSynthConfig>) {
    let volume_delta = 0.1;

    for mut config in &mut query {
        if keyboard.just_pressed(KeyCode::Equal) {
            config.volume = (config.volume + volume_delta).min(1.0);
        }
        if keyboard.just_pressed(KeyCode::Minus) {
            config.volume = (config.volume - volume_delta).max(0.0);
        }
    }
}

/// System for debug logging of MIDI commands
pub fn debug_midi_commands(query: Query<(Entity, &MidiCommands), Changed<MidiCommands>>) {
    for (entity, commands) in &query {
        for command in &commands.queue {
            info!("Entity {:?} MIDI command: {:?}", entity, command);
        }
    }
}

/// Bundle for easily creating a MIDI synthesizer entity
#[derive(Bundle)]
pub struct MidiSynthBundle {
    pub soundfont: MidiSoundfont,
    pub commands: MidiCommands,
    pub config: MidiSynthConfig,
}

impl MidiSynthBundle {
    /// Create a new MIDI synth bundle with the given soundfont
    pub fn new(soundfont: Handle<SoundFont>) -> Self {
        Self {
            soundfont: MidiSoundfont { handle: soundfont },
            commands: MidiCommands::default(),
            config: MidiSynthConfig::default(),
        }
    }

    /// Create a new MIDI synth bundle with custom configuration
    pub fn with_config(soundfont: Handle<SoundFont>, config: MidiSynthConfig) -> Self {
        Self {
            soundfont: MidiSoundfont { handle: soundfont },
            commands: MidiCommands::default(),
            config,
        }
    }
}

/// System set for MIDI-related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MidiSystemSet {
    /// Systems that send MIDI commands
    Input,
    /// Systems that process MIDI commands
    Processing,
    /// Systems that handle audio output
    Output,
}
