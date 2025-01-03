use reader::ReaderError;

use crate::prelude::*;

use super::MidiFile;

#[derive(Default)]
pub enum FormatStage<'a> {
    #[default]
    Unknown,
    KnownFormat(RawFormat<'a>),
    KnownTracks(Vec<Track<'a>>),
    Formatted(Format<'a>),
}

#[derive(Default)]
pub struct MidiFileBuilder<'a> {
    format: FormatStage<'a>,
    timing: Option<Timing<'a>>,
    unknown_chunks: Vec<UnknownChunk<'a>>,
}

impl<'a> MidiFileBuilder<'a> {
    pub fn handle_chunk<'b: 'a>(&mut self, chunk: ChunkEvent<'b>) -> ReadResult<()> {
        use ChunkEvent::*;
        match chunk {
            Header(h) => {
                if self.timing.is_some() {
                    return Err(ReaderError::invalid_data(
                        "Found another header, should only expect one",
                    ));
                }

                match self.format {
                    FormatStage::Unknown => {
                        self.format = FormatStage::KnownFormat(h.format().clone());
                    }
                    FormatStage::KnownFormat(_) | FormatStage::Formatted(_) => {
                        return Err(ReaderError::invalid_data(
                            "Found another format when one was already provided",
                        ));
                    }
                    FormatStage::KnownTracks(ref tracks) => match h.format_type() {
                        FormatType::Simultaneous => {
                            self.format =
                                FormatStage::Formatted(Format::Simultaneous(tracks.clone()))
                        }
                        FormatType::SingleMultiChannel => {
                            // this shouldn't even happen...but we will support headers that aren't at the top of the file, so it *could*
                            if tracks.len() != 1 {
                                return Err(ReaderError::invalid_data(
                                    "track lengths is greater than one, yet format is single multichannel",
                                ));
                            }
                            let track = tracks.first().unwrap().clone();
                            self.format = FormatStage::Formatted(Format::SingleMultiChannel(track))
                        }
                        FormatType::SequentiallyIndependent => {
                            self.format = FormatStage::Formatted(Format::SequentiallyIndependent(
                                tracks.clone(),
                            ))
                        }
                    },
                };

                self.timing = Some(h.timing().clone());

                Ok(())
            }
            Track(t) => {
                let events = t.events()?;

                let track = super::Track::new(events);
                match &mut self.format {
                    FormatStage::Unknown => {
                        self.format = FormatStage::KnownTracks(vec![track]);
                    }
                    FormatStage::KnownFormat(t) => match t.format_type() {
                        FormatType::Simultaneous => {
                            self.format = FormatStage::Formatted(Format::Simultaneous(vec![track]))
                        }
                        FormatType::SingleMultiChannel => {
                            self.format = FormatStage::Formatted(Format::SingleMultiChannel(track))
                        }
                        FormatType::SequentiallyIndependent => {
                            self.format =
                                FormatStage::Formatted(Format::SequentiallyIndependent(vec![track]))
                        }
                    },
                    FormatStage::KnownTracks(tracks) => tracks.push(track),
                    FormatStage::Formatted(format) => match format {
                        Format::SequentiallyIndependent(tracks) => tracks.push(track),
                        Format::SingleMultiChannel(_) => {
                            return Err(ReaderError::invalid_data(
                                "Track of format 0 has multiple tracks",
                            ));
                        }
                        Format::Simultaneous(tracks) => tracks.push(track),
                    },
                }
                Ok(())
            }
            Unknown(data) => {
                self.unknown_chunks.push(data);
                Ok(())
            }
            EOF => Err(ReaderError::oob("Expected end of file to be handled")),
        }
    }
    pub fn build(self) -> ReadResult<MidiFile<'a>> {
        let FormatStage::Formatted(format) = self.format else {
            return Err(ReaderError::invalid_data(
                "Error: format doesn't line up with tracks",
            ));
        };
        let Some(timing) = self.timing else {
            return Err(ReaderError::invalid_data("No timing provided"));
        };

        Ok(MidiFile {
            format,
            header: Header::new(timing),
        })
    }
}
