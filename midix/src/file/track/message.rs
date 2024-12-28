use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessage<'a> {
    ChannelVoice(ChannelVoice<'a>),
    SystemExclusive(SysEx<'a>),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(Meta<'a>),
}
