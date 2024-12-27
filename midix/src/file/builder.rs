use crate::file::{ReadResult, ReaderError};
use crate::prelude::*;

#[derive(Default)]
pub enum FormatStage<'a> {
    #[default]
    Unknown,
    KnownType(MidiFormatRef<'a>),
    KnownTracks(Vec<MidiTrack>),
    Formatted(MidiFormat),
}

impl FormatStage<'_> {
    pub fn known(&self) -> bool {
        !(matches!(self, Self::Unknown))
    }
}

#[derive(Default)]
pub struct MidiFileBuilder<'a> {
    format: FormatStage<'a>,
    timing: Option<MidiTiming>,
}

impl<'a> MidiFileBuilder<'a> {
    pub fn handle_chunk<'b: 'a>(&mut self, chunk: MidiChunk<'b>) -> ReadResult<()> {
        use MidiChunk::*;
        match chunk {
            Header(h) => {
                if self.timing.is_some() {
                    return Err(ReaderError::invalid_data());
                }

                match self.format {
                    FormatStage::Unknown => {
                        self.format = FormatStage::KnownType(h.format());
                    }
                    FormatStage::KnownType(_) | FormatStage::Formatted(_) => {
                        return Err(ReaderError::invalid_data());
                    }
                    FormatStage::KnownTracks(ref t) => match h.format_type() {
                        MidiFormatType::Simultaneous => {
                            self.format =
                                FormatStage::Formatted(MidiFormat::Simultaneous(t.clone()))
                        }
                        MidiFormatType::SingleMultiChannel => {
                            if t.len() != 1 {
                                return Err(ReaderError::invalid_data());
                            }
                            let track = t.first().unwrap().clone();
                            self.format =
                                FormatStage::Formatted(MidiFormat::SingleMultiChannel(track))
                        }
                        MidiFormatType::SequentiallyIndependent => {
                            self.format = FormatStage::Formatted(
                                MidiFormat::SequentiallyIndependent(t.clone()),
                            )
                        }
                    },
                };

                self.timing = Some(h.timing().to_owned());

                Ok(())
            }
            Track(t) => {
                let events = t.events()?;

                for event in events {}
                todo!();
            }

            _ => todo!(),
        }
    }
}
