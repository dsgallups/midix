use midir::MidiInputPort;

use super::{MidiData, MidiInputError};

pub enum MidirReply {
    AvailablePorts(Vec<(String, MidiInputPort)>),
    Error(MidiInputError),
    Connected,
    Disconnected,
    Midi(MidiData),
}

impl MidirReply {
    pub fn dbg(&self) -> String {
        use MidirReply::*;
        match self {
            MidirReply::AvailablePorts(ports) => {
                let mut res = "Available Ports:\n".to_string();
                for (name, port) in ports.iter() {
                    res.push_str(&format!("Name - {}, port id- {}", name, port.id()));
                }
                res
            }
            Error(e) => {
                format!("Error({e:?}")
            }
            Connected => "Connected".to_string(),
            Disconnected => "Disconnected".to_string(),
            Midi(_) => "Midi Data".to_string(),
        }
    }
}
