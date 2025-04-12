use bevy::prelude::*;
use crossbeam_channel::{Receiver, TryRecvError};
use midix::events::{FromLiveEventBytes, LiveEvent};

use crate::synth::Synth;

use super::MidiInputError;

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, handle_mididata);
}

/// An [`Event`] for incoming midi data.
#[derive(Event, Debug)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: Option<u64>,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}
#[derive(Component)]
pub struct MidiInputConnection {
    data: Receiver<MidiData>,
    conn: Option<midir::MidiInputConnection<()>>,
}

impl MidiInputConnection {
    pub fn new(
        midir_input: midir::MidiInput,
        port_id: String,
        port_name: &str,
    ) -> Result<Self, MidiInputError> {
        let (sender, receiver) = crossbeam_channel::unbounded::<MidiData>();
        let Some(port) = midir_input.find_port_by_id(port_id.clone()) else {
            return Err(MidiInputError::port_not_found(port_id));
        };

        let conn = midir_input.connect(
            &port,
            port_name,
            {
                move |timestamp, data, _| {
                    let Ok(message) = LiveEvent::from_bytes(data) else {
                        return;
                    };
                    sender
                        .send(MidiData {
                            stamp: Some(timestamp),
                            message,
                        })
                        .unwrap();
                }
            },
            (),
        )?;

        Ok(Self {
            data: receiver,
            conn: Some(conn),
        })
    }
    pub fn read(&self) -> Result<MidiData, TryRecvError> {
        self.data.try_recv()
    }
    pub fn close(&mut self) {
        if let Some(conn) = self.conn.take() {
            conn.close();
        };
    }
}

//todo: move this to the example
fn handle_mididata(connections: Query<&MidiInputConnection>, synth: Res<Synth>) {
    for connection in connections {
        while let Ok(data) = connection.read() {
            let LiveEvent::ChannelVoice(event) = data.message else {
                continue;
            };
            //todo
            synth.handle_event(event);
        }
    }
}
