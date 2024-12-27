use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessage {
    ChannelVoice(ChannelVoice),
    SystemExclusive(SysEx),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(Meta),
}
