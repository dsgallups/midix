use crate::file::{ReadResult, ReaderError};
use crate::prelude::*;

use super::MidiFile;

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
    unknown_chunks: Vec<Vec<u8>>,
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
                    FormatStage::KnownTracks(ref tracks) => match h.format_type() {
                        MidiFormatType::Simultaneous => {
                            self.format =
                                FormatStage::Formatted(MidiFormat::Simultaneous(tracks.clone()))
                        }
                        MidiFormatType::SingleMultiChannel => {
                            if tracks.len() != 1 {
                                return Err(ReaderError::invalid_data());
                            }
                            let track = tracks.first().unwrap().clone();
                            self.format =
                                FormatStage::Formatted(MidiFormat::SingleMultiChannel(track))
                        }
                        MidiFormatType::SequentiallyIndependent => {
                            self.format = FormatStage::Formatted(
                                MidiFormat::SequentiallyIndependent(tracks.clone()),
                            )
                        }
                    },
                };

                self.timing = Some(h.timing().to_owned());

                Ok(())
            }
            Track(t) => {
                let events = t.events()?.into_iter().map(|e| e.to_owned()).collect();

                let track = MidiTrack::new(events);
                match self.format {
                    FormatStage::Unknown => {
                        self.format = FormatStage::KnownTracks(vec![track]);
                    }
                    FormatStage::KnownType(t) => match t.format_type() {
                        MidiFormatType::Simultaneous => {
                            self.format =
                                FormatStage::Formatted(MidiFormat::Simultaneous(vec![track]))
                        }
                        MidiFormatType::SingleMultiChannel => {
                            self.format =
                                FormatStage::Formatted(MidiFormat::SingleMultiChannel(track))
                        }
                        MidiFormatType::SequentiallyIndependent => {
                            self.format =
                                FormatStage::Formatted(MidiFormat::SequentiallyIndependent(vec![
                                    track,
                                ]))
                        }
                    },
                    FormatStage::KnownTracks(ref mut tracks) => tracks.push(track),
                    FormatStage::Formatted(ref mut format) => match format {
                        MidiFormat::SequentiallyIndependent(tracks) => tracks.push(track),
                        MidiFormat::SingleMultiChannel(_) => {
                            return Err(ReaderError::invalid_data());
                        }
                        MidiFormat::Simultaneous(tracks) => tracks.push(track),
                    },
                }
                Ok(())
            }
            Unknown { data, .. } => {
                self.unknown_chunks.push(data.to_vec());
                Ok(())
            }
        }
    }
    pub fn build(self) -> ReadResult<MidiFile> {
        let FormatStage::Formatted(f) = self.format else {
            return Err(ReaderError::invalid_data());
        };
        let Some(timing) = self.timing else {
            return Err(ReaderError::invalid_data());
        };

        Ok(MidiFile {
            tracks: f,
            header: MidiHeader::new(timing),
        })
    }
}
