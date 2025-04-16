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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MidiMessageBytes {
    /// Message is only one byte
    Status(StatusByte),

    /// Message is a [`StatusByte`] and a [`DataByte`]
    Single(StatusByte, DataByte),

    /// Message is a [`StatusByte`] and two [`DataByte`]s
    Double(StatusByte, DataByte, DataByte),
}

// impl Read for MidiMessageBytes {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         use MidiMessageBytes::*;
//         match self {
//             Status(s) => {
//                 let Some(byte) = buf.get_mut(0) else {
//                     return Ok(0);
//                 };
//                 *byte = s.0;
//                 Ok(1)
//             }
//             Single(s, d) => {
//                 let Some(byte) = buf.get_mut(0) else {
//                     return Ok(0);
//                 };
//                 *byte = s.0;
//                 let Some(byte) = buf.get_mut(1) else {
//                     return Ok(1);
//                 };
//                 *byte = d.0;
//                 Ok(2)
//             }
//             Double(s, d1, d2) => {
//                 let Some(byte) = buf.get_mut(0) else {
//                     return Ok(0);
//                 };
//                 *byte = s.0;
//                 let Some(byte) = buf.get_mut(1) else {
//                     return Ok(1);
//                 };
//                 *byte = d1.0;

//                 let Some(byte) = buf.get_mut(1) else {
//                     return Ok(2);
//                 };
//                 *byte = d2.0;
//                 Ok(3)
//             }
//         }
//     }
// }

impl MidiMessageBytes {
    /// Write the contents of self into some writer as MIDI bytes.
    ///
    /// Returns number of bytes written.
    pub fn write_to_writer<W: Write + ?Sized>(&self, writer: &mut W) -> Result<usize, io::Error> {
        use MidiMessageBytes::*;
        match self {
            Status(s) => {
                writer.write_all(&[s.0])?;
                Ok(1)
            }
            Single(s, d) => {
                writer.write_all(&[s.0, d.0])?;
                Ok(2)
            }
            Double(s, d1, d2) => {
                writer.write_all(&[s.0, d1.0, d2.0])?;
                Ok(3)
            }
        }
    }

    /// Create a MidiMessageByte from a single status byte. Errors if leading 1 is not found.
    pub fn from_status<B, E>(status: B) -> Result<Self, io::Error>
    where
        B: TryInto<StatusByte, Error = E>,
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusByte(u8);

impl Debug for StatusByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusByte(0x{:0X})", self.0)
    }
}

impl StatusByte {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Only use if the value is already been checked or
    /// constructed such that it cannot have a leading 0 bit
    pub(crate) fn new_unchecked(byte: u8) -> Self {
        Self(byte)
    }

    /// Get the underlying byte of the status
    pub fn byte(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for StatusByte {
    type Error = io::Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(&byte)
            .then_some(Self(byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<Cow<'a, u8>> for StatusByte {
    type Error = io::Error;
    fn try_from(byte: Cow<'a, u8>) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(byte.as_ref())
            .then_some(Self(*byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<&'a u8> for StatusByte {
    type Error = io::Error;
    fn try_from(byte: &'a u8) -> Result<Self, Self::Error> {
        (0x80..=0xFF)
            .contains(byte)
            .then_some(Self(*byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl fmt::Display for StatusByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.0)
    }
}

#[doc = r#"
Data Byte is between [0x00 and 0x7F]
"#]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataByte(pub(crate) u8);
impl Debug for DataByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug(0x{:0X}", self.0)
    }
}

impl DataByte {
    /// Check a new status byte
    pub fn new(byte: u8) -> Result<Self, io::Error> {
        byte.try_into()
    }

    /// Create a data byte without checking for the leading 0.
    pub const fn new_unchecked(byte: u8) -> Self {
        Self(byte)
    }

    /// Get the underlying byte of the data
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for DataByte {
    type Error = io::Error;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(&byte)
            .then_some(Self(byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<&'a u8> for DataByte {
    type Error = io::Error;
    fn try_from(byte: &'a u8) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(byte)
            .then_some(Self(*byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl<'a> TryFrom<Cow<'a, u8>> for DataByte {
    type Error = io::Error;
    fn try_from(byte: Cow<'a, u8>) -> Result<Self, Self::Error> {
        (0x00..=0x7F)
            .contains(byte.as_ref())
            .then_some(Self(*byte))
            .ok_or(io::Error::new(
                ErrorKind::InvalidData,
                "Expected Status byte",
            ))
    }
}

impl fmt::Display for DataByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}", self.0)
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
