use crate::file::{ReadResult, ReaderError};
use crate::prelude::*;

#[derive(Default)]
pub enum FormatStage {
    #[default]
    Unknown,
    KnownType(MidiFormatType),
    Formatted(MidiFormat),
}

impl FormatStage {
    pub fn known(&self) -> bool {
        !(matches!(self, Self::Unknown))
    }
}

#[derive(Default)]
pub struct MidiFileBuilder<'a> {
    header: Option<MidiHeader>,
    format: FormatStage,

    store: Option<MidiHeaderRef<'a>>,
}

impl<'a> MidiFileBuilder<'a> {
    pub fn handle_chunk<'b: 'a>(&mut self, chunk: MidiChunk<'b>) -> ReadResult<()> {
        use MidiChunk::*;
        match chunk {
            Header(h) => {
                if self.store.is_some() || self.header.is_some() || self.format.known() {
                    return Err(ReaderError::invalid_data());
                }
                self.format = FormatStage::KnownType(h.format_type());

                self.store = Some(h);

                todo!()
            }

            _ => todo!(),
        }
    }
}
