#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormatRef<'a> {
    /// Format 0
    SingleMultiChannel,
    /// Format 1
    Simultaneous(&'a [u8; 2]),
    /// Format 2
    SequentiallyIndependent(&'a [u8; 2]),
}

impl MidiFormatRef<'_> {
    pub fn num_tracks(self) -> u16 {
        use MidiFormatRef::*;
        match self {
            SingleMultiChannel => 1,
            Simultaneous(num) | SequentiallyIndependent(num) => u16::from_be_bytes(*num),
        }
    }
    pub fn format_type(&self) -> MidiFormatType {
        use MidiFormatRef::*;
        match self {
            SingleMultiChannel => MidiFormatType::SingleMultiChannel,
            Simultaneous(_) => MidiFormatType::Simultaneous,
            SequentiallyIndependent(_) => MidiFormatType::SequentiallyIndependent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiFormatType {
    SingleMultiChannel,
    Simultaneous,
    SequentiallyIndependent,
}
