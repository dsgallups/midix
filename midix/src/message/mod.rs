#![doc = r#"
Contains all possible message types per MIDI 1.0

There are five total message types. The layout looks like this:
- [`MidiMessage`]
    - [`SystemMessage`]
        - [`SystemCommonMessage`]
        - [`SystemRealTimeMessage`]
        - [`SystemExclusiveMessage`]
    - [`ChannelMessage`]
        - [`ChannelVoiceMessage`]
        - [`ChannelModeMessage`]

# Hierarchy
```text
                |--------------|
                | MIDI Message |
                |--------------|
                 /            \
|-----------------|          |----------------|
| Channel Message |          | System Message |
|-----------------|          |----------------|
```

```text
                |--------------------------|
                |      System Message      |
                |--------------------------|
                 /           |            \
|----------------| |-------------------| |-------------------|
| Common Message | | Real-time Message | | Exclusive Message |
|----------------| |-------------------| |-------------------|
```

```text
                |-----------------|
                | Channel Message |
                |-----------------|
                 /               \
|-----------------------|   |----------------------|
| Channel Voice Message |   | Channel Mode Message |
|-----------------------|   |----------------------|
```

"#]

pub mod channel;
pub use channel::*;
pub mod system;
pub use system::*;

#[doc = r#"
An enumeration of all possible midi messages
"#]
#[derive(Debug)]
pub enum MidiMessage<'a> {
    /// A system common message
    SysCommon(SystemCommonMessage<'a>),

    /// A system real-time message
    SysRealTime(SystemRealTimeMessage),

    /// A system exclusive message
    SysExclusive(SystemExclusiveMessage<'a>),

    /// A channel voice message
    ChannelVoice(ChannelVoiceMessage),

    /// A channel mode message
    ChannelMode(ChannelModeMessage),
}

impl<'a> From<SystemMessage<'a>> for MidiMessage<'a> {
    fn from(value: SystemMessage<'a>) -> Self {
        match value {
            SystemMessage::Common(c) => MidiMessage::SysCommon(c),
            SystemMessage::Exclusive(e) => MidiMessage::SysExclusive(e),
            SystemMessage::RealTime(r) => MidiMessage::SysRealTime(r),
        }
    }
}

impl From<ChannelMessage> for MidiMessage<'_> {
    fn from(value: ChannelMessage) -> Self {
        match value {
            ChannelMessage::Mode(m) => MidiMessage::ChannelMode(m),
            ChannelMessage::Voice(v) => MidiMessage::ChannelVoice(v),
        }
    }
}
impl<'a> From<SystemCommonMessage<'a>> for MidiMessage<'a> {
    fn from(value: SystemCommonMessage<'a>) -> Self {
        Self::SysCommon(value)
    }
}

impl From<SystemRealTimeMessage> for MidiMessage<'_> {
    fn from(value: SystemRealTimeMessage) -> Self {
        Self::SysRealTime(value)
    }
}

impl<'a> From<SystemExclusiveMessage<'a>> for MidiMessage<'a> {
    fn from(value: SystemExclusiveMessage<'a>) -> Self {
        Self::SysExclusive(value)
    }
}

impl From<ChannelVoiceMessage> for MidiMessage<'_> {
    fn from(value: ChannelVoiceMessage) -> Self {
        Self::ChannelVoice(value)
    }
}

impl From<ChannelModeMessage> for MidiMessage<'_> {
    fn from(value: ChannelModeMessage) -> Self {
        Self::ChannelMode(value)
    }
}
