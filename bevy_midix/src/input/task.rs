use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, TryRecvError};
pub use midir::MidiInputPort;
use midix::events::{FromLiveEventBytes, LiveEvent};

use crate::input::{MidiData, MidiInputError};

use super::{MidiInputSettings, MidirReply};

pub(crate) enum UserMessage {
    RefreshPorts,
    ConnectToPort(MidiInputPort),
    DisconnectFromPort,
}

// Returns either MidirReply::AvailablePorts or MidirReply::PortRefreshError
// If there's an error getting port names, it's because the available ports changed,
// so it tries again (up to 10 times)
fn get_available_ports(input: &midir::MidiInput) -> MidirReply {
    for _ in 0..10 {
        let ports = input.ports();
        let ports: Result<Vec<_>, _> = ports
            .into_iter()
            .map(|p| input.port_name(&p).map(|n| (n, p)))
            .collect();
        if let Ok(ports) = ports {
            return MidirReply::AvailablePorts(ports);
        }
    }
    MidirReply::Error(MidiInputError::PortRefreshError)
}

pub(crate) struct MidiInputTask {
    pub receiver: Receiver<UserMessage>,
    pub sender: Sender<MidirReply>,
    pub settings: MidiInputSettings,
    // Invariant: exactly one of `input` or `connection` is Some
    pub input: Option<midir::MidiInput>,
    pub connection: Option<(midir::MidiInputConnection<()>, MidiInputPort)>,
}

impl Future for MidiInputTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.input.is_none() && self.connection.is_none() {
            self.input = midir::MidiInput::new(self.settings.client_name).ok();
            self.sender
                .send(get_available_ports(self.input.as_ref().unwrap()))
                .unwrap();
        }

        let msg = match self.receiver.try_recv() {
            Ok(msg) => msg,
            Err(TryRecvError::Disconnected) => {
                return Poll::Ready(());
            }
            Err(TryRecvError::Empty) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        match msg {
            UserMessage::ConnectToPort(port) => {
                let was_connected = self.input.is_none();
                let s = self.sender.clone();
                let i = self
                    .input
                    .take()
                    .unwrap_or_else(|| self.connection.take().unwrap().0.close().0);
                let conn = i.connect(
                    &port,
                    self.settings.port_name,
                    move |stamp, message, _| {
                        let Ok(message) = LiveEvent::from_bytes(message) else {
                            return;
                        };
                        let data = MidiData {
                            stamp: Some(stamp),
                            message,
                        };
                        let _ = s.send(MidirReply::Midi(data));
                    },
                    (),
                );
                match conn {
                    Ok(conn) => {
                        self.sender.send(MidirReply::Connected).unwrap();
                        self.connection = Some((conn, port));
                        self.input = None;
                    }
                    Err(conn_err) => {
                        self.sender
                            .send(MidirReply::Error(MidiInputError::ConnectionError(
                                conn_err.kind(),
                            )))
                            .unwrap();
                        if was_connected {
                            self.sender.send(MidirReply::Disconnected).unwrap();
                        }
                        self.connection = None;
                        self.input = Some(conn_err.into_inner());
                    }
                }
            }
            UserMessage::DisconnectFromPort => {
                if let Some((conn, _)) = self.connection.take() {
                    self.input = Some(conn.close().0);
                    self.connection = None;
                    self.sender.send(MidirReply::Disconnected).unwrap();
                }
            }
            UserMessage::RefreshPorts => match &self.input {
                Some(i) => {
                    self.sender.send(get_available_ports(i)).unwrap();
                }
                None => {
                    let (conn, port) = self.connection.take().unwrap();
                    let i = conn.close().0;

                    self.sender.send(get_available_ports(&i)).unwrap();

                    let s = self.sender.clone();
                    let conn = i.connect(
                        &port,
                        self.settings.port_name,
                        move |stamp, message, _| {
                            let Ok(message) = LiveEvent::from_bytes(message) else {
                                return;
                            };
                            let data = MidiData {
                                stamp: Some(stamp),
                                message,
                            };
                            let _ = s.send(MidirReply::Midi(data));
                        },
                        (),
                    );
                    match conn {
                        Ok(conn) => {
                            self.connection = Some((conn, port));
                            self.input = None;
                        }
                        Err(conn_err) => {
                            self.sender
                                .send(MidirReply::Error(MidiInputError::ConnectionError(
                                    conn_err.kind(),
                                )))
                                .unwrap();
                            self.sender.send(MidirReply::Disconnected).unwrap();
                            self.connection = None;
                            self.input = Some(conn_err.into_inner());
                        }
                    }
                }
            },
        }
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}
