use std::{
    borrow::Cow,
    io::{self, ErrorKind, Write},
};

#[doc = r#"
Identifies a byte that follows the MIDI spec:

Status Byte
(80H - FFH)

or

Data Byte
(00H - 7FH)
"#]
#[allow(dead_code)]
pub struct MidiByte<'a>(Cow<'a, u8>);

#[doc = r#"
There are only three types of midi message bytes:

```text
1.  |--------|
    | Status |
    |--------|

2.  |--------|   |------|
    | Status | - | Data |
    |--------|   |------|

3.  |--------|   |------|   |------|
    | Status | - | Data | - | Data |
    |--------|   |------|   |------|
```
"#]
pub enum MidiMessageBytes<'a> {
    /// Message is only one byte
    Status(StatusByte<'a>),

    /// Message is a [`StatusByte`] and a [`DataByte`]
    Single(StatusByte<'a>, DataByte<'a>),

    /// Message is a [`StatusByte`] and two [`DataByte`]s
    Double(StatusByte<'a>, DataByte<'a>, DataByte<'a>),
}

impl MidiMessageBytes<'_> {
    /// Write the contents of self into some writer as MIDI bytes
    pub fn write<W: Write + ?Sized>(&self, writer: &mut W) -> Result<(), io::Error> {
        use MidiMessageBytes::*;
        match self {
            Status(s) => writer.write_all(&[*s.0]),
            Single(s, d) => writer.write_all(&[*s.0, *d.0]),
            Double(s, d1, d2) => writer.write_all(&[*s.0, *d1.0, *d2.0]),
        }
    }
}

#[doc = r#"
Status Byte is between [0x80 and 0xFF]


Status bytes are eight-bit binary numbers in which the Most Significant Bit (MSB) is set (binary 1).
Status bytes serve to identify the message type, that is, the purpose of the Data bytes which follow it.
Except for Real-Time messages, new Status bytes will always command a receiver to adopt a new status,
even if the last message was not completed.
"#]
pub struct StatusByte<'a>(Cow<'a, u8>);

impl<'a> StatusByte<'a> {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        (0x80..=0xFF)
            .contains(&byte)
            .then_some(Self(Cow::Owned(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }

    /// Check a reference to a status byte
    pub fn new_borrowed(byte: &'a u8) -> Result<Self, io::Error> {
        (0x80..=0xFF)
            .contains(byte)
            .then_some(Self(Cow::Borrowed(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

#[doc = r#"
Data Byte is between [0x00 and 0x7F]
"#]
pub struct DataByte<'a>(Cow<'a, u8>);

impl<'a> DataByte<'a> {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        (0x80..=0xFF)
            .contains(&byte)
            .then_some(Self(Cow::Owned(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }

    /// Check a reference to a status byte
    pub fn new_borrowed(byte: &'a u8) -> Result<Self, io::Error> {
        (0x00..=0x7F)
            .contains(byte)
            .then_some(Self(Cow::Borrowed(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

#[doc = r#"
Any types that can be represented as a `MidiMessageByte`.

Notable, [`SystemExclusiveMessage`] and [`SystemRealTimeMessage`]
do not implement this trait. They have separate structure types
"#]
pub trait MidiMessageByteRep<'a> {
    /// Represent oneself as MidiMessageBytes.
    fn as_midi_bytes(&self) -> MidiMessageBytes<'a>;
}

impl<'a, W, T> MidiWriteable<W> for T
where
    W: Write + ?Sized,
    T: MidiMessageByteRep<'a>,
{
    /// Writes the byte representation of the type into a writer
    fn write_into(&self, writer: &mut W) -> Result<(), io::Error> {
        self.as_midi_bytes().write(writer)
    }
}

#[doc = r#"
Any representation that can be written, as bytes, into some writer
"#]
pub trait MidiWriteable<W: Write + ?Sized> {
    /// Writes the byte representation of the type into a writer
    fn write_into(&self, writer: &mut W) -> Result<(), io::Error>;
}

#[doc = r#"
A trait for things that can write to midi.

# Overview
Why not use [`Write`](std::io::Write) instead?

Unfortunately, MIDI events have different byte representations depending on whether it's streamed or
written out to smf format.
"#]
pub trait MidiWriter {
    fn write_midi(&mut self, byte: &[u8]);
}