use crate::prelude::*;

#[doc = r#"
An enumerationg of categories which may be

Track Messages fall into three categories:
- [`ChannelVoice`]: Notes, velocities, pedals, channel events.
- [`SystemExclusive`]: Inaudible events communicated between devices
- [`Meta`]: Identifiers for the track, like name, copyright information, arbitrary text.
"#]
#[derive(Clone, Debug, PartialEq)]
pub enum TrackMessage<'a> {
    /// A channel voice message.
    ///
    /// See [`ChannelVoice`] for details
    ChannelVoice(ChannelVoice<'a>),

    /// A system exclusive event.
    ///
    /// See [`SysEx`] for details
    SystemExclusive(SysEx<'a>),

    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    ///
    /// See [`Meta`] for details
    Meta(Meta<'a>),
}
