use bevy::prelude::*;
use bevy_seedling::{node::Events, prelude::*};
use firewheel::event::NodeEventType;

use crate::prelude::SoundFont;

mod components;
pub use components::*;
mod node;
pub use node::*;

/// Plugin for MIDI synthesis using Firewheel/bevy_seedling
pub struct FirewheelMidiPlugin;

impl Plugin for FirewheelMidiPlugin {
    fn build(&self, app: &mut App) {
        // Register our custom node type with bevy_seedling
        // Since MidiSynthNode doesn't implement Diff/Patch, we use register_simple_node
        app.register_simple_node::<MidiSynthNode>();

        // Initialize soundfont assets
        #[cfg(feature = "std")]
        {
            app.init_asset::<SoundFont>()
                .init_asset_loader::<crate::bevy::asset::SoundFontLoader>();
        }

        // Add our systems
        app.add_systems(Update, (spawn_midi_nodes, process_midi_commands).chain());
    }
}

/// System that spawns MIDI synthesizer nodes for entities with soundfonts
#[allow(clippy::type_complexity)]
fn spawn_midi_nodes(
    mut commands: Commands,
    soundfont_assets: Res<Assets<SoundFont>>,
    query: Query<
        (Entity, &MidiSoundfont, Option<&MidiSynthConfig>),
        (Without<FirewheelNode>, With<MidiCommands>),
    >,
) {
    for (entity, soundfont, config) in &query {
        // Check if soundfont is loaded
        let Some(soundfont_asset) = soundfont_assets.get(&soundfont.0) else {
            continue;
        };

        // Get config or use defaults
        let config = config.cloned().unwrap_or_default();

        // Create node configuration
        let node_config = MidiSynthNodeConfig {
            soundfont: soundfont_asset.file.clone(),
            sample_rate: 44100.0, // This will be overridden by the audio context
            enable_reverb: config.enable_reverb,
            enable_chorus: config.enable_chorus,
        };

        // Create the node with the initial volume
        let node = MidiSynthNode {
            volume: config.volume,
        };

        // Add the node and its configuration to the entity
        // bevy_seedling will automatically handle node creation and connection
        commands.entity(entity).insert((node, node_config));
    }
}

/// System that processes MIDI commands and sends them to the audio nodes
fn process_midi_commands(mut query: Query<(&FirewheelNode, &mut MidiCommands, &mut Events)>) {
    for (_, mut commands, mut events) in &mut query {
        if commands.queue.is_empty() {
            continue;
        }

        // Take all pending commands
        let pending = commands.take();

        // Send commands to the audio node as custom events
        for command in pending {
            events.push(NodeEventType::Custom(Box::new(command)));
        }
    }
}

/// Extension trait for Commands to easily spawn MIDI synths
pub trait MidiCommandsExt {
    /// Spawn a MIDI synthesizer with the given soundfont
    fn spawn_midi_synth(&mut self, soundfont: Handle<SoundFont>) -> EntityCommands<'_>;

    /// Spawn a MIDI synthesizer with custom configuration
    fn spawn_midi_synth_with_config(
        &mut self,
        soundfont: Handle<SoundFont>,
        config: MidiSynthConfig,
    ) -> Entity;
}

impl MidiCommandsExt for Commands<'_, '_> {
    fn spawn_midi_synth(&mut self, soundfont: Handle<SoundFont>) -> EntityCommands<'_> {
        self.spawn((
            MidiSoundfont(soundfont),
            MidiCommands::default(),
            MidiSynthConfig::default(),
            Name::new("MIDI Synthesizer"),
        ))
    }

    fn spawn_midi_synth_with_config(
        &mut self,
        soundfont: Handle<SoundFont>,
        config: MidiSynthConfig,
    ) -> Entity {
        self.spawn((
            MidiSoundfont(soundfont),
            MidiCommands::default(),
            config,
            Name::new("MIDI Synthesizer"),
        ))
        .id()
    }
}
