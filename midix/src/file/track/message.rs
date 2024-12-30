use crate::prelude::*;

#[doc = r#"
An enumerationg of categories which may be

Track Messages fall into three categories:
- [`ChannelVoiceMessage`]: Notes, velocities, pedals, channel events.
- [`SystemExclusiveMessage`]: Inaudible events communicated between devices
- ['MetaMessage']: Identifiers for the track, like name, copyright information, arbitrary text.
"#]
#[derive(Clone, Debug, PartialEq)]
pub enum TrackMessage<'a> {
    /// A channel voice message.
    ///
    /// See [`ChannelVoiceMessage`] for details
    ChannelVoice(ChannelVoiceMessage<'a>),

    /// A system exclusive event.
    ///
    /// See [`SystemExclusiveMessage`] for details
    SystemExclusive(SystemExclusiveMessage<'a>),

    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    ///
    /// See [`MetaMessage`] for details
    Meta(MetaMessage<'a>),
}
