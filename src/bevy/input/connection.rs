use crate::events::{FromLiveEventBytes, LiveEvent};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, TryRecvError};
use midir::MidiInputPort;

use super::MidiInputError;

/// An [`Event`] for incoming midi data.
#[derive(Event, Debug)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: Option<u64>,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}
pub(crate) struct MidiInputConnection {
    data: Receiver<MidiData>,
    conn: midir::MidiInputConnection<()>,
}

impl MidiInputConnection {
    pub fn new(
        midir_input: midir::MidiInput,
        port: &MidiInputPort,
        port_name: &str,
    ) -> Result<Self, MidiInputError> {
        let (sender, receiver) = crossbeam_channel::unbounded::<MidiData>();

        let conn = midir_input.connect(
            port,
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
            conn,
        })
    }
    pub fn read(&self) -> Result<MidiData, TryRecvError> {
        self.data.try_recv()
    }
    pub fn close(self) -> midir::MidiInput {
        let (listener, _) = self.conn.close();
        listener
    }
}
