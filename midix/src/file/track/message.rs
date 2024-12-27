use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MidiTrackMessage {
    ChannelVoice(ChannelVoiceMessage),
    SystemExclusive(SystemExclusive),
    /// A meta-message, giving extra information for correct playback, like tempo, song name,
    /// lyrics, etc...
    Meta(MetaMessage),
}
