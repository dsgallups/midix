use std::io::Read;

use bevy::prelude::*;
use midir::{MidiOutputPort, SendError};
use midix::MidiMessageBytes;

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
        //TODO: don't unwrap
        let wrote = message.read(&mut buf).unwrap();
        self.conn.send(&buf[..wrote])?;
        Ok(())
    }
    pub fn close(self) -> midir::MidiOutput {
        self.conn.close()
    }
}
