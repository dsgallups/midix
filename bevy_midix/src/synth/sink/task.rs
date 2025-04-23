#[cfg(not(feature = "web"))]
use std::time::{Duration, Instant};
use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};
#[cfg(feature = "web")]
use web_time::{Duration, Instant};

use bevy::log::info;
/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use midix::prelude::*;

use super::{MidiSong, SinkCommand, SongId, SongType, inner::InnerCommand};

#[derive(Default)]
pub struct CommandQueue(VecDeque<InnerCommand>);

impl CommandQueue {
    fn queue_commands(
        &mut self,
        id: Option<SongId>,
        events: impl IntoIterator<Item = Timed<ChannelVoiceMessage>>,
        elapsed: u64,
    ) {
        for message in events {
            let amt = elapsed + message.timestamp;

            info!(
                "Message will be played in {}",
                Duration::from_micros(amt).as_secs_f64()
            );
            self.push_back(InnerCommand {
                time_to_send: amt,
                parent: id,
                command: message.event,
            });
        }
    }
    fn push_back(&mut self, command: InnerCommand) {
        self.0.push_back(command);
    }
    fn sort(&mut self) {
        self.0.make_contiguous().sort_by_key(|m| m.time_to_send);
    }
    fn front(&self) -> Option<&InnerCommand> {
        self.0.front()
    }
    fn pop_front(&mut self) -> Option<InnerCommand> {
        self.0.pop_front()
    }
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&InnerCommand) -> bool,
    {
        self.0.retain(f);
    }
}

/// This struct is essentially the glue
/// that determines when to send messages to the synthesizer.
///
/// It needs its own thread because it's going to need to update its timer
///
/// as frequently as possible.
pub(crate) struct SinkTask {
    start: Instant,
    synth_channel: Sender<ChannelVoiceMessage>,
    commands: Receiver<SinkCommand>,
    queue: CommandQueue,
    /// Stored songs that are looping
    keepsakes: Vec<SongInfo>,
}

struct SongInfo {
    song: MidiSong,
    last_repeated: Instant,
    length: u64,
}

impl SinkTask {
    pub fn new(
        synth_channel: Sender<ChannelVoiceMessage>,
        commands: Receiver<SinkCommand>,
    ) -> Self {
        Self {
            start: Instant::now(),
            synth_channel,
            commands,
            queue: CommandQueue::default(),
            keepsakes: Vec::new(),
        }
    }

    // make sure the commands are already sorted.
    fn keep(&mut self, song: MidiSong) {
        let length = song.events.last().map(|e| e.timestamp).unwrap_or(0);
        self.keepsakes.push(SongInfo {
            song,
            last_repeated: Instant::now(),
            length,
        })
    }

    // song commands should already be sorted.
    //
    // elapsed is in micros
    fn queue_commands(
        &mut self,
        id: Option<SongId>,
        events: impl IntoIterator<Item = Timed<ChannelVoiceMessage>>,
        elapsed: u64,
    ) {
        for message in events {
            let amt = elapsed + message.timestamp;

            info!(
                "Message will be played in {}",
                Duration::from_micros(amt).as_secs_f64()
            );
            self.queue.push_back(InnerCommand {
                time_to_send: amt,
                parent: id,
                command: message.event,
            });
        }
    }
}

impl Future for SinkTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut new_messages_pushed = false;
        // first check if there's anything in the queue
        let elapsed = self.start.elapsed().as_micros() as u64;
        while let Some(messages) = match self.commands.try_recv() {
            Ok(m) => Some(m),
            Err(e) => match e {
                TryRecvError::Disconnected => {
                    return Poll::Ready(());
                }
                _ => None,
            },
        } {
            match messages {
                SinkCommand::NewSong {
                    song_type,
                    mut commands,
                } => {
                    if commands.is_empty() {
                        continue;
                    }
                    commands.sort_by_key(|m| m.timestamp);

                    new_messages_pushed = true;
                    if let SongType::Identified { id, looped } = song_type {
                        self.keep(MidiSong {
                            id,
                            events: commands.clone(),
                            looped,
                        });
                    }

                    self.queue_commands(song_type.id(), commands, elapsed);
                }
                SinkCommand::Stop {
                    song_id,
                    stop_voices,
                } => {
                    if let Some(song_id) = song_id {
                        self.queue
                            .retain(|command| command.parent.is_none_or(|id| id != song_id));
                        self.keepsakes.retain(|info| info.song.id != song_id);
                    }
                    if stop_voices {
                        let events = Channel::all().into_iter().map(|channel| {
                            Timed::new(
                                0,
                                ChannelVoiceMessage::new(
                                    channel,
                                    VoiceEvent::control_change(Controller::mute_all()),
                                ),
                            )
                        });
                        self.queue_commands(None, events, elapsed);
                    }
                }
            }

            //do something
        }
        if new_messages_pushed {
            self.queue.sort();
        }

        let elapsed = self.start.elapsed().as_micros() as u64;

        while self
            .queue
            .front()
            .is_some_and(|first| first.time_to_send <= elapsed)
        {
            let message = self.queue.pop_front().unwrap();

            info!(
                "({}) {:?}",
                message.command.channel(),
                message.command.event()
            );

            self.synth_channel.send(message.command).unwrap();
        }

        //finally, queue any songs that have elapsed their length
        let mut songs_to_clone = Vec::new();
        for info in self.keepsakes.iter_mut() {
            if info.last_repeated.elapsed().as_micros() as u64 >= info.length {
                songs_to_clone.push((Some(info.song.id), info.song.events.clone()));
                info.last_repeated = Instant::now();
            }
        }
        for (id, iter) in songs_to_clone {
            self.queue.queue_commands(id, iter, elapsed);
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
