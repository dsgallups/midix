use bevy::prelude::*;
use midir::{MidiInputConnection, MidiInputPort};

use super::MidiInputError;

enum MidiConnectionInner {
    Listening(midir::MidiInput),
    Connected(MidiInputConnection<()>),
}

#[derive(Resource)]
pub struct MidiConnection {
    client_name: &'static str,
    inner: Option<MidiConnectionInner>,
}

impl MidiConnection {
    pub fn new(client_name: &'static str) -> Self {
        let input = midir::MidiInput::new(client_name).unwrap();
        Self {
            client_name,
            inner: Some(MidiConnectionInner::Listening(input)),
        }
    }
    pub fn listening(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|i| matches!(i, MidiConnectionInner::Listening(_)))
    }
    pub fn connected(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|i| matches!(i, MidiConnectionInner::Connected(_)))
    }
    pub fn ports(&self) -> Option<Vec<MidiInputPort>> {
        self.inner.as_ref().and_then(|i| {
            let MidiConnectionInner::Listening(i) = i else {
                return None;
            };
            Some(i.ports())
        })
    }

    /// drops any active connection
    pub fn start_listening(&mut self) {
        self.inner = Some(MidiConnectionInner::Listening(
            midir::MidiInput::new(self.client_name).unwrap(),
        ));
    }

    /// The callback will handle a timestamp and midi message.
    pub fn connect<F>(
        &mut self,
        port: &MidiInputPort,
        port_name: &str,
        mut callback: F,
    ) -> Result<(), MidiInputError>
    where
        F: FnMut(u64, &[u8]) + Send + 'static,
    {
        if self
            .inner
            .as_ref()
            .is_none_or(|i| matches!(i, MidiConnectionInner::Connected(_)))
        {
            return Err(MidiInputError::invalid("There is no listener!"));
        };
        let MidiConnectionInner::Listening(listener) = self.inner.take().unwrap() else {
            unreachable!()
        };
        let connection = listener
            .connect(
                port,
                port_name,
                move |timestamp, data, _| callback(timestamp, data),
                (),
            )
            .unwrap();

        self.inner = Some(MidiConnectionInner::Connected(connection));
        Ok(())
    }
}
