use crate::{
    channel::ChannelId,
    events::LiveEvent,
    prelude::{BytesText, Tempo, TimeSignature, TrackEvent, TrackMessage},
};

#[doc = r#"
A set of track events
"#]
#[derive(Debug, Clone, PartialEq)]
pub struct Track<'a> {
    info: TrackInfo<'a>,
    events: Vec<TimedEvent<LiveEvent<'a>>>,
}

impl<'a> Track<'a> {
    /// Create a new track
    pub fn new(events: Vec<TrackEvent<'a>>) -> Self {
        let mut info = TrackInfo::default();
        let mut track_events = Vec::with_capacity(events.len());

        let mut time_accumulated = None;

        for event in events {
            let delta_ticks = event.delta_ticks();

            let accumulated_ticks = if let Some(mut tick_acc) = &mut time_accumulated {
                tick_acc += delta_ticks;
                tick_acc
            } else {
                time_accumulated = Some(delta_ticks);
                delta_ticks
            };
            let event: LiveEvent = match event.into_event() {
                TrackMessage::ChannelVoice(cvm) => cvm.into(),
                TrackMessage::SystemExclusive(sysex) => sysex.into(),
                TrackMessage::Meta(meta) => {
                    meta.adjust_track_info(&mut info);
                    continue;
                }
            };
            track_events.push(TimedEvent::new(accumulated_ticks, event));
        }

        // update track_event's time_since_start, since it currently
        // holds delta_time, which is fractions of a beat.
        // So we want to convert fractions of a beat to microseconds
        // The conversion is
        // Track (us / quarter note) *
        // Header (quarter notes / tick )^-1
        // Ticks (delta time)

        Self {
            info,
            events: track_events,
        }
    }
}

/// Provides information about the track
#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TrackInfo<'a> {
    pub time_signature: TimeSignature<'a>,
    pub name: Option<BytesText<'a>>,
    pub device: Option<BytesText<'a>>,
    pub track_info: Option<u16>,
    pub channel: Option<ChannelId>,
    pub tempo: Tempo,
}

#[doc = r#"
A wrapper around some type with an associated time
"#]
#[derive(Debug, Clone, PartialEq)]
pub struct TimedEvent<T> {
    /// In microseconds. Can be ticks, or microseconds
    accumulated_ticks: u32,
    event: T,
}

impl<T> TimedEvent<T> {
    /// Create a new timed event based on *accumulated* ticks
    pub fn new(accumulated_ticks: u32, event: T) -> Self {
        Self {
            accumulated_ticks,
            event,
        }
    }
}
