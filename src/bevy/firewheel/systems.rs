use crate::bevy::asset::{MidiFile, SoundFont};
use crate::bevy::firewheel::components::*;
use crate::prelude::*;
use bevy::prelude::*;

/// System for playing a MIDI file through a synthesizer
pub fn play_midi_file(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    midi_assets: Res<Assets<MidiFile>>,
    mut query: Query<(&Handle<MidiFile>, &mut MidiCommands), Added<Handle<MidiFile>>>,
) {
    for (midi_handle, mut commands) in &mut query {
        if let Some(midi_file) = midi_assets.get(midi_handle) {
            // Convert MIDI file to song and extract commands
            let song = midi_file.to_song();

            // Send all MIDI events as commands
            for timed_event in song.events() {
                commands.send(timed_event.event);
            }
        }
    }
}

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
                commands.send(ChannelVoiceMessage::note_on(0, note, 100));
            }
            if keyboard.just_released(key) {
                // Send note off
                commands.send(ChannelVoiceMessage::note_off(0, note, 0));
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
                commands.send(ChannelVoiceMessage::note_off(0, prev_note, 0));
            }

            // Play current note
            let note = scale[*note_index % scale.len()];
            commands.send(ChannelVoiceMessage::note_on(0, note, 80));

            *note_index += 1;
        }
    }
}

/// System for sending program change (instrument selection)
pub fn set_instrument(
    mut query: Query<(&mut MidiCommands, &MidiInstrument), Changed<MidiInstrument>>,
) {
    for (mut commands, instrument) in &mut query {
        commands.send(ChannelVoiceMessage::program_change(
            instrument.channel,
            instrument.program,
        ));
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
            // Send all notes off on all channels
            for channel in 0..16 {
                commands.send(ChannelVoiceMessage::all_notes_off(channel));
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
