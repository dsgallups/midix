use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum LiveEventError {
//     #[error("Invalid slice length: {0}")]
//     InvalidLength(u8),
//     #[error("Invalid Data Byte: {0:0X}")]
//     InvalidDataByte(u8),
//     #[error("Invalid Status: {0:0X}")]
//     InvalidStatusByte(u8),
//     #[error("Required data not found at indice {0}")]
//     DataNotFound(usize),
// }

// ///TODO
// impl From<ParseError> for LiveEventError {
//     fn from(value: ParseError) -> Self {
//         match value {
//             ParseError::InvalidDataByte(u) => LiveEventError::InvalidLength(u),
//             ParseError::InvalidStatusByte(u) => LiveEventError::InvalidStatusByte(u),
//             _ => unreachable!(),
//         }
//     }
// }

/// All the ways parsing can go wrong
#[derive(Debug, Error)]
pub enum ParseError {
    /// Invalid databyte (leading 1)
    #[error("Invalid Data Byte: {0:0X}")]
    InvalidDataByte(u8),
    #[error("Invalid Status: {0:0X}")]
    /// Invalid status byte
    InvalidStatusByte(u8),
    #[error("Invalid length: {0}")]
    /// Desired length for message not found
    InvalidLength(usize),
    #[error("Smpte: {0}")]
    Smpte(#[from] SmpteError),
    /// There's something missing from the thing being parsed
    #[error("Expected Data")]
    MissingData,
    /// Invalid system common message byte
    #[error("Invalid System Common Message: {0:0X}")]
    InvalidSystemCommonMessage(u8),
    /// This can happen in meta messages
    #[error("Invalid Channel in MetaMessage: {0:0X}")]
    InvalidChannel(u8),
    /// What's supposed to be utf8, is in fact, not
    #[error("Invalid UTF8")]
    InvalidUtf8,
    /// the varlen for parsing channel count is invalid
    #[error("Meta Message: {0}")]
    MetaMessage(MetaMessageError),
    /// Something wrong with a file's header
    #[error("Header: {0}")]
    Header(HeaderError),
    /// Something wrong with a file chunk
    #[error("{0}")]
    Chunk(ChunkError),
    /// Something wrong with a file track
    #[error("{0}")]
    TrackEvent(TrackError),
    /// Errors that occur after all reading has completed
    #[error("{0}")]
    File(FileError),
}
impl ParseError {
    pub(crate) fn channel_count(varlen: usize) -> Self {
        Self::MetaMessage(MetaMessageError::ChannelCount(varlen))
    }
    pub(crate) fn port(varlen: usize) -> Self {
        Self::MetaMessage(MetaMessageError::Port(varlen))
    }
    pub(crate) fn time_sig(varlen: usize) -> Self {
        Self::MetaMessage(MetaMessageError::TimeSignature(varlen))
    }
    pub(crate) fn key_sig(varlen: usize) -> Self {
        Self::MetaMessage(MetaMessageError::KeySignature(varlen))
    }
}
/// Problems reading a file's header
#[derive(Debug, Error)]
pub enum HeaderError {
    /// Type 0 MIDI Format (SingleMultiChannel) defines multiple tracks
    #[error("Type 0 MIDI Format (SingleMultiChannel) defined multiple tracks")]
    MultiTracksInSingleMultiChannel,
    /// Byte isn't 0 (SingleMultiChannel), 1, (Simultaneous), or 2 (Sequentially Independent)
    #[error(
        "Byte isn't 0 (SingleMultiChannel), 1, (Simultaneous), or 2 (Sequentially Independent). Found {0}"
    )]
    InvalidMidiFormat(u8),
    /// The timing type is not defined
    #[error("Timing type isn't 0 (TicksPerQuarterNote) or 1 (NegativeSmpte). Found {0}")]
    InvalidTiming(u8),
    /// Invalid header chunk size (RawHeaderChunk needs this to be 6)
    #[error(
        "Invalid header chunk size. Expected to be of length 6. Cannot determine length found."
    )]
    InvalidSize,
}
impl From<HeaderError> for ParseError {
    fn from(value: HeaderError) -> Self {
        Self::Header(value)
    }
}

/// Problems reading a file's metamessages
#[derive(Debug, Error)]
pub enum MetaMessageError {
    /// contains varlen inner, should be 1.
    #[error("varlen for channel count was {0}. Expected 1.")]
    ChannelCount(usize),
    /// contains varlen inner. Should be 1.
    #[error("varlen for port was {0}. Expected 1.")]
    Port(usize),
    /// contains varlen inner. Should be 4.
    #[error("varlen for time signature was {0}. Expected 4.")]
    TimeSignature(usize),
    /// contains varlen inner. Should be 2.
    #[error("varlen for key signature was {0}. Expected 2.")]
    KeySignature(usize),
}
impl From<MetaMessageError> for ParseError {
    fn from(value: MetaMessageError) -> Self {
        Self::MetaMessage(value)
    }
}

/// problems reading a track
#[derive(Debug, Error)]
pub enum TrackError {
    /// Invalid event
    #[error("Invalid event found: {0:0X}")]
    InvalidEvent(u8),
}

impl From<TrackError> for ParseError {
    fn from(value: TrackError) -> Self {
        Self::TrackEvent(value)
    }
}

/// Problems reading a chunk
#[derive(Debug, Error)]
pub enum ChunkError {
    /// Finding more than one header for a chunk
    #[error("Found more than one header for this chunk.")]
    DuplicateHeader,
    /// Finding more than one format for a chunk
    #[error("Found more than one format after one was provided")]
    DuplicateFormat,
    /// the format is singlemultichannel, yet multiple tracks were found
    #[error("There's more than one track for a SingleMultiChannel format")]
    MultipleTracksForSingleMultiChannel,
}

/// Problems with the file after reading it through
#[derive(Debug, Error)]
pub enum FileError {
    /// No format was found
    #[error("The file's format couldn't be determined")]
    NoFormat,
    /// No timing was found
    #[error("The file has no timing")]
    NoTiming,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SmpteError {
    #[error("Invalid hour for offset. Expected 0-24. Got {0}")]
    HourOffset(u8),
    #[error("Invalid minute for offset. Expected 0-59. Got {0}")]
    MinuteOffset(u8),
    #[error("Invalid second for offset. Expected 0-59. Got {0}")]
    SecondOffset(u8),
    #[error("Invalid frame for offset. Expected 0-(n frames - 1). Got {0}")]
    FrameOffset(u8),
    #[error("Invalid subframe offset. Subframes are 1/100th (0-99) of a frame. Got {0}")]
    Subframe(u8),
    #[error("Smpte meta length for track invalid (always must be 5): {0}")]
    Length(usize),
    #[error("Invalid frame for track interpretation. Should be 0, 1, 2, or 3. Got 0")]
    TrackFrame(u8),

    #[error("Invalid SMPTE time in header (only -24, -25, -29, and -30 allowed.) Interpreted {0}")]
    HeaderFrameTime(i8),
}
