#![doc = r#"
Contains all system common message types

# Hierarchy
```text
                |--------------------------|
                |      System Message      |
                |--------------------------|
                 /           |            \
|----------------| |-------------------| |-------------------|
| Common Message | | Real-time Message | | Exclusive Message |
|----------------| |-------------------| |-------------------|
```
"#]

mod common;
pub use common::*;

mod realtime;
pub use realtime::*;

mod exclusive;
pub use exclusive::*;

#[doc = r#"
The set of possible System messages
"#]
pub enum SystemMessage<'a> {
    /// The set of common messages. Only found in MIDI files
    Common(SystemCommonMessage<'a>),

    /// The set of common messages. Only found in
    /// [`LiveEvent`](crate::prelude::LiveEvent)s
    RealTime(SystemRealTimeMessage),

    /// The catch-all System Exclusive Message. Found
    /// both in files and in live events.
    Exclusive(SystemExclusiveMessage<'a>),
}

impl<'a> From<SystemCommonMessage<'a>> for SystemMessage<'a> {
    fn from(value: SystemCommonMessage<'a>) -> Self {
        SystemMessage::Common(value)
    }
}

impl From<SystemRealTimeMessage> for SystemMessage<'_> {
    fn from(value: SystemRealTimeMessage) -> Self {
        SystemMessage::RealTime(value)
    }
}

impl<'a> From<SystemExclusiveMessage<'a>> for SystemMessage<'a> {
    fn from(value: SystemExclusiveMessage<'a>) -> Self {
        SystemMessage::Exclusive(value)
    }
}
