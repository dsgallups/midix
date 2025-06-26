use core::fmt::{self, Debug};

use crate::prelude::*;

#[doc = r#"
An enumerationg of categories which may be

Track Messages fall into three categories:
- [`ChannelVoiceMessage`]: Notes, velocities, pedals, channel events.
- [`SystemExclusiveMessage`]: Inaudible events communicated between devices
- ['MetaMessage']: Identifiers for the track, like name, copyright information, arbitrary text.
"#]
#[derive(Clone, PartialEq, Eq)]
pub enum TrackMessage<'a> {
    /// A channel voice message.
    ///
    /// See [`ChannelVoiceMessage`] for details
    ChannelVoice(ChannelVoiceMessage),

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

impl Debug for TrackMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChannelVoice(c) => {
                write!(f, "{c:?}")
            }
            Self::SystemExclusive(s) => {
                write!(f, "{s:?}")
            }
            Self::Meta(m) => {
                write!(f, "{m:?}")
            }
        }
    }
}

impl From<ChannelVoiceMessage> for TrackMessage<'_> {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}

impl<'a> From<SystemExclusiveMessage<'a>> for TrackMessage<'a> {
    fn from(value: SystemExclusiveMessage<'a>) -> Self {
        Self::SystemExclusive(value)
    }
}

impl<'a> From<MetaMessage<'a>> for TrackMessage<'a> {
    fn from(value: MetaMessage<'a>) -> Self {
        Self::Meta(value)
    }
}
