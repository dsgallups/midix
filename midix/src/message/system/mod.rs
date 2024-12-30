#![doc = r#"
A list of system common messages

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
