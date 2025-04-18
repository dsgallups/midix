use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

/*

This Sink will send events to another thread that will constantly poll/flush command out to the synth.
*/
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use midix::prelude::ChannelVoiceMessage;

use super::{SinkCommands, inner::InnerCommand};

/// This struct is essentially the glue
/// that determines when to send messages to the synthesizer.
///
/// It needs its own thread because it's going to need to update its timer
///
/// as frequently as possible.
pub(crate) struct SinkTask {
    now: Instant,
    accumulated_time: u64,
    synth_channel: Sender<ChannelVoiceMessage>,
    commands: Receiver<SinkCommands>,
    queue: VecDeque<InnerCommand>,
}

impl SinkTask {
    pub fn new(
        synth_channel: Sender<ChannelVoiceMessage>,
        commands: Receiver<SinkCommands>,
    ) -> Self {
        Self {
            now: Instant::now(),
            accumulated_time: 0,
            synth_channel,
            commands,
            queue: VecDeque::new(),
        }
    }
}

impl Future for SinkTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
        // first check if there's anything in the queue
        let accumulated_time = self.accumulated_time;
        while let Some(mut messages) = match self.commands.try_recv() {
            Ok(m) => Some(m),
            Err(e) => match e {
                TryRecvError::Disconnected => {
                    panic!("synth disconnected");
                }
                _ => None,
            },
        } {
            messages.0.sort_by_key(|m| m.timestamp);
            self.queue
                .extend(messages.0.into_iter().map(|message| InnerCommand {
                    time_to_send: accumulated_time + message.timestamp,
                    command: message.event,
                }));
            //do something
        }
        let earlier = self.now;
        self.now = Instant::now();
        let time_passed = self.now.duration_since(earlier);
        // 584542 years is more than enough space
        self.accumulated_time += time_passed.as_micros() as u64;

        while self
            .queue
            .front()
            .is_some_and(|first| first.time_to_send <= self.accumulated_time)
        {
            let message = self.queue.pop_front().unwrap();

            self.synth_channel.send(message.command).unwrap();
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
