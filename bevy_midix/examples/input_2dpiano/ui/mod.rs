use bevy::{
    color::palettes::{
        css::{GREEN, RED},
        tailwind::{YELLOW_200, YELLOW_300},
    },
    prelude::*,
};
use bevy_midix::prelude::*;

mod menu;
mod piano;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UiState {
    #[default]
    Listening,
    Active,
}

pub fn plugin(app: &mut App) {
    app.init_state::<UiState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(PostUpdate, sync_state)
        .add_systems(OnEnter(UiState::Listening), menu::spawn_connect_ui)
        .add_systems(OnExit(UiState::Listening), menu::cleanup)
        .add_systems(
            Update,
            menu::update_available_ports.run_if(in_state(UiState::Listening)),
        )
        .add_systems(OnEnter(UiState::Active), piano::spawn_piano)
        .add_systems(
            Update,
            piano::handle_input.run_if(in_state(UiState::Active)),
        );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn sync_state(input: Res<MidiInput>, mut state: ResMut<NextState<UiState>>, mut last: Local<bool>) {
    let currently_active = input.is_active();
    if currently_active == *last {
        return;
    }
    if input.is_active() {
        state.set(UiState::Active)
    } else {
        state.set(UiState::Listening)
    }
    *last = currently_active;
}
