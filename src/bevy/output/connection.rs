use crate::MidiMessageBytes;
use bevy::prelude::*;
use midir::{MidiOutputPort, SendError};

use super::MidiOutputError;

pub(crate) struct MidiOutputConnection {
    conn: midir::MidiOutputConnection,
}

impl MidiOutputConnection {
    pub fn new(
        midir_input: midir::MidiOutput,
        port: &MidiOutputPort,
        port_name: &str,
    ) -> Result<Self, MidiOutputError> {
        let conn = midir_input.connect(port, port_name)?;

        Ok(Self { conn })
    }
    pub fn send(&mut self, message: impl Into<MidiMessageBytes>) -> Result<(), SendError> {
        let mut buf = [0; 3];
        let mut message: MidiMessageBytes = message.into();
        let wrote = message.write_into(&mut buf);
        self.conn.send(&buf[..wrote])?;
        Ok(())
    }
    pub fn close(self) -> midir::MidiOutput {
        self.conn.close()
    }
}
