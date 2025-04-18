use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use bevy::log::info;
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
    start: Instant,
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
            start: Instant::now(),
            synth_channel,
            commands,
            queue: VecDeque::new(),
        }
    }
}

impl Future for SinkTask {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut messages_pushed = false;
        // first check if there's anything in the queue
        let elapsed = self.start.elapsed().as_micros() as u64;
        while let Some(mut messages) = match self.commands.try_recv() {
            Ok(m) => Some(m),
            Err(e) => match e {
                TryRecvError::Disconnected => {
                    return Poll::Ready(());
                }
                _ => None,
            },
        } {
            messages.0.sort_by_key(|m| m.timestamp);

            for message in messages.0 {
                let amt = elapsed + message.timestamp;

                info!(
                    "Message will be played in {}",
                    Duration::from_micros(amt).as_secs_f64()
                );
                self.queue.push_back(InnerCommand {
                    time_to_send: amt,
                    command: message.event,
                });
                messages_pushed = true;
            }

            //do something
        }
        if messages_pushed {
            self.queue.make_contiguous().sort_by_key(|m| m.time_to_send);
        }

        let elapsed = self.start.elapsed().as_micros() as u64;

        while self
            .queue
            .front()
            .is_some_and(|first| first.time_to_send <= elapsed)
        {
            let message = self.queue.pop_front().unwrap();
            info!("Queueing {:?}", message.command);

            self.synth_channel.send(message.command).unwrap();
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
