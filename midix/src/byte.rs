use std::{
    borrow::Cow,
    fmt::{self, Debug},
    io::{self, ErrorKind, Write},
};

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

impl<'a> MidiMessageBytes<'a> {
    /// Write the contents of self into some writer as MIDI bytes
    pub fn write<W: Write + ?Sized>(&self, writer: &mut W) -> Result<(), io::Error> {
        use MidiMessageBytes::*;
        match self {
            Status(s) => writer.write_all(&[*s.0]),
            Single(s, d) => writer.write_all(&[*s.0, *d.0]),
            Double(s, d1, d2) => writer.write_all(&[*s.0, *d1.0, *d2.0]),
        }
    }

    /// Create a MidiMessageByte from a single status byte. Errors if leading 1 is not found.
    pub fn from_status<B, E>(status: B) -> Result<Self, io::Error>
    where
        B: TryInto<StatusByte<'a>, Error = E>,
        E: Into<io::Error>,
    {
        let status = status.try_into().map_err(Into::into)?;
        Ok(Self::Status(status))
    }
}

#[doc = r#"
Status Byte is between [0x80 and 0xFF]


Status bytes are eight-bit binary numbers in which the Most Significant Bit (MSB) is set (binary 1).
Status bytes serve to identify the message type, that is, the purpose of the Data bytes which follow it.
Except for Real-Time messages, new Status bytes will always command a receiver to adopt a new status,
even if the last message was not completed.
"#]
#[derive(Clone)]
pub struct StatusByte<'a>(Cow<'a, u8>);

impl Debug for StatusByte<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusByte(0x{:0X})", *self.0)
    }
}

impl<'a> StatusByte<'a> {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Only use if the value is already been checked or
    /// constructed such that it cannot have a leading 0 bit
    pub(crate) fn new_unchecked(byte: u8) -> Self {
        Self(Cow::Owned(byte))
    }

    /// Check a reference to a status byte
    pub fn new_borrowed(byte: &'a u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Get the underlying byte of the status
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl TryFrom<u8> for StatusByte<'_> {
    type Error = io::Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(&byte)
            .then_some(Self(Cow::Owned(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<Cow<'a, u8>> for StatusByte<'a> {
    type Error = io::Error;
    fn try_from(byte: Cow<'a, u8>) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(byte.as_ref())
            .then_some(Self(byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<&'a u8> for StatusByte<'a> {
    type Error = io::Error;
    fn try_from(byte: &'a u8) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(byte)
            .then_some(Self(Cow::Borrowed(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl PartialEq for StatusByte<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}
impl Eq for StatusByte<'_> {}
impl fmt::Display for StatusByte<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.0.as_ref())
    }
}

#[doc = r#"
Data Byte is between [0x00 and 0x7F]
"#]
#[derive(Clone, Hash)]
pub struct DataByte<'a>(Cow<'a, u8>);
impl Debug for DataByte<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug(0x{:0X}", *self.0)
    }
}

impl<'a> DataByte<'a> {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Check a reference to a status byte
    pub fn new_borrowed(byte: &'a u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Create a data byte without checking for the leading 0.
    pub const fn new_unchecked(byte: u8) -> Self {
        Self(Cow::Owned(byte))
    }

    /// Create a referenced data byte without checking for the leading 0.
    pub const fn new_borrowed_unchecked(byte: &'a u8) -> Self {
        Self(Cow::Borrowed(byte))
    }

    /// Get the underlying byte of the data
    pub fn byte(&self) -> &u8 {
        &self.0
    }
}

impl TryFrom<u8> for DataByte<'_> {
    type Error = io::Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(&byte)
            .then_some(Self(Cow::Owned(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<&'a u8> for DataByte<'a> {
    type Error = io::Error;
    fn try_from(byte: &'a u8) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(byte)
            .then_some(Self(Cow::Borrowed(byte)))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<Cow<'a, u8>> for DataByte<'a> {
    type Error = io::Error;
    fn try_from(byte: Cow<'a, u8>) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(byte.as_ref())
            .then_some(Self(byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl PartialEq for DataByte<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}
impl Eq for DataByte<'_> {}
impl fmt::Display for DataByte<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.0.as_ref())
    }
}

/* TODO: planned
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
*/
