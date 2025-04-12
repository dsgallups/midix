use bevy::prelude::*;
use midir::MidiInputPort;

#[derive(Resource, Default)]
pub struct MidiInputPorts {
    pub(crate) ports: Vec<MidiInputPort>,
}
impl MidiInputPorts {
    pub fn ports(&self) -> &[MidiInputPort] {
        &self.ports
    }
}
