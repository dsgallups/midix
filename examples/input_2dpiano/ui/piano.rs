use std::collections::{HashMap, VecDeque};

use bevy::{
    color::palettes::{
        css::{GREEN, RED},
        tailwind::YELLOW_500,
    },
    prelude::*,
    reflect::List,
};
use itertools::Itertools;
use midix::prelude::*;

use crate::ExampleInputEvent;

#[derive(Component)]
pub struct Piano;

#[derive(Component)]
pub struct CommandText;
#[derive(Component)]
pub struct InfoText;

pub fn spawn_piano(mut commands: Commands, asset_server: Res<AssetServer>) {
    let get_note = |i: u8| {
        use Note::*;
        match i % 12 {
            0 => A,
            1 => ASharp,
            2 => B,
            3 => C,
            4 => CSharp,
            5 => D,
            6 => DSharp,
            7 => E,
            8 => F,
            9 => FSharp,
            10 => G,
            _ => GSharp,
        }
    };
    let get_octave = |i: u8| {
        // piano octave starts at 0.
        //
        // note that octaves start with C, not A. so that's why we add 9 here.
        let octave = (i + 9) / 12;
        Octave::new(octave as i8)
    };
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            Node {
                top: Val::Percent(25.),

                ..Default::default()
            },
            InfoText,
            Text::default(),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextSpan::new("Press ESC to disconnect\n"),
                TextFont {
                    font: font.clone(),
                    font_size: 30.,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            commands.spawn((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    font_size: 10.,
                    ..default()
                },
                TextColor(Color::WHITE),
                CommandText,
            ));
        });

    commands
        .spawn((
            Piano,
            Node {
                padding: UiRect::top(Val::Px(5.))
                    .with_right(Val::Px(5.))
                    .with_left(Val::Px(5.)),
                box_sizing: BoxSizing::BorderBox,
                width: Val::Percent(100.),
                height: Val::Px(80.),
                ..default()
            },
        ))
        .with_children(|commands| {
            (0..88).for_each(|i| {
                // Note: there's an easier way to do this.
                // In theory, you can make the key by adding an amount to i such that
                // the first key is A:0 using `Key::from_databyte`.
                // Note that there are 127 available keys.
                // The first key (byte 0) is going to be C:-1. so you'd write
                //
                // `let key = Key::from_databyte(i + 9 + 12).unwrap()`
                //
                // 9 will get the next A, and 12 skips the -1 octave. most soundfonts
                // don't carry a -1 piano octave.
                //
                // However, for example purposes, I have provided the logic for getting
                // the correct key via note and octave.
                let note = get_note(i);
                let octave = get_octave(i);
                let key = Key::new(note, octave);

                commands
                    .spawn((
                        Button,
                        Node {
                            flex_grow: 1.,
                            border: UiRect::all(Val::Px(1.)),
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BackgroundColor(bg_color(key.is_sharp())),
                        key,
                    ))
                    .observe(on_mouse_leave)
                    .observe(on_mouse_up);
            })
        });
}
pub fn cleanup(
    mut commands: Commands,
    piano: Query<Entity, With<Piano>>,
    text: Query<Entity, With<InfoText>>,
) {
    let piano = piano.single().unwrap();
    commands.entity(piano).despawn();
    let text = text.single().unwrap();
    commands.entity(text).despawn();
}

fn bg_color(sharp: bool) -> Color {
    if sharp { Color::BLACK } else { Color::WHITE }
}

const HOVERED: Srgba = GREEN;
const PRESSED: Srgba = RED;

// use mouse input over Interaction::Pressed so you can hold down the button and go nuts
pub fn handle_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut keys: Query<(&Interaction, &mut BackgroundColor, &Key), Changed<Interaction>>,
    synth: Res<Synth>,
) {
    for (interaction, mut background_color, key) in &mut keys {
        match *interaction {
            Interaction::Hovered | Interaction::Pressed => {
                if mouse_input.pressed(MouseButton::Left) {
                    warn!("{key} pressed");
                    *background_color = PRESSED.into();
                    let event =
                        VoiceEvent::note_on(*key, Velocity::MAX).send_to_channel(Channel::One);
                    _ = synth.push_event(event);
                } else {
                    *background_color = HOVERED.into();
                }
            }
            // on_mouse_leave does runs logic that would otherwise be here.
            Interaction::None => {}
        }
    }
}

pub fn handle_midi_device_input(
    mut ev: EventReader<ExampleInputEvent>,

    mut keys: Query<(&mut BackgroundColor, &Key)>,
) {
    let mut key_events = HashMap::new();
    for event in ev.read() {
        // we use this functionality because Note On with a velocity of zero is note off.
        let is_note_on = event.voice.is_note_on();
        let is_note_off = event.voice.is_note_off();
        if !is_note_on && !is_note_off {
            continue;
        }
        let key = match event.voice {
            VoiceEvent::NoteOn { key, .. } | VoiceEvent::NoteOff { key, .. } => key,
            _ => continue,
        };
        key_events.insert(key, is_note_on);
    }
    if key_events.is_empty() {
        return;
    }
    keys.par_iter_mut().for_each(|(mut background_color, key)| {
        let Some(is_note_on) = key_events.get(key) else {
            return;
        };
        if *is_note_on {
            *background_color = YELLOW_500.into();
        } else {
            *background_color = bg_color(key.is_sharp()).into();
        }
    });
}

pub fn update_command_text(
    mut ev: EventReader<ExampleInputEvent>,
    mut command_text: Query<&mut TextSpan, With<CommandText>>,
    mut all_cmds: Local<VecDeque<String>>,
) {
    if ev.is_empty() {
        return;
    }
    let mut command_text = command_text.single_mut().unwrap();
    for event in ev.read() {
        let val = match event.voice {
            VoiceEvent::NoteOn { key, velocity } => {
                format!("Note On: {key} with {velocity} velocity")
            }
            VoiceEvent::NoteOff { key, velocity } => {
                format!("Note Off: {key} with {velocity} velocity")
            }
            VoiceEvent::PitchBend(pb) => {
                format!("Pitch Bend: {pb:?}")
            }
            VoiceEvent::Aftertouch { key, velocity } => {
                format!("AfterTouch: {key} with {velocity} velocity")
            }
            VoiceEvent::ProgramChange { program } => {
                format!("Program Change: {program:?}")
            }
            VoiceEvent::ChannelPressureAfterTouch { velocity } => {
                format!("ChannelPressure after touch: {velocity} velocity")
            }
            VoiceEvent::ControlChange(controller) => {
                format!("Control Change: {controller:?}")
            }
        };
        all_cmds.push_front(val);
    }
    while all_cmds.len() > 60 {
        all_cmds.pop();
    }
    let formatted = all_cmds.iter().join("\n");
    command_text.0 = formatted;
}

// handles the case where you are dragging and then you release the mouse on a key.
// The Interaction component will remain Interaction::Hovered, so the handle_input system will not catch it.
fn on_mouse_up(
    trigger: Trigger<Pointer<Released>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    synth: Res<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    warn!("{key} unpressed");

    let event = VoiceEvent::note_on(*key, Velocity::ZERO).send_to_channel(Channel::One);
    // could make this beter and revert to hover, but lazy
    *background_color = HOVERED.into();
    _ = synth.push_event(event);
}

// because Interaction::Pressed doesn't do anything if you leave pressed.
fn on_mouse_leave(
    trigger: Trigger<Pointer<Out>>,
    mut keys: Query<(&mut BackgroundColor, &Key)>,
    synth: Res<Synth>,
) {
    let (mut background_color, key) = keys.get_mut(trigger.target()).unwrap();
    *background_color = BackgroundColor(bg_color(key.is_sharp()));
    let event = VoiceEvent::note_on(*key, Velocity::ZERO).send_to_channel(Channel::One);
    _ = synth.push_event(event);
}
