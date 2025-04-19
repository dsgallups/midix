use crate::{ParseError, SmpteError, prelude::SmpteFrame};

pub struct SmtpeOffset {}

impl SmtpeOffset {
    pub fn parse(data: &[u8]) -> Result<Self, SmpteError> {
        if data.len() != 5 {
            return Err(SmpteError::Length(data.len()));
        }
        let hour_byte = data[0];

        // 0 rr hhhhh
        let frame = match hour_byte >> 5 {
            0 => SmpteFrame::TwentyFour,
            1 => SmpteFrame::TwentyFive,
            2 => SmpteFrame::TwentyNine,
            3 => SmpteFrame::Thirty,
            v => return Err(SmpteError::TrackFrame(v)),
        };
        let hour = hour_byte & 0b0001_1111;
        if hour > 24 {
            return Err(SmpteError::HourOffset(hour_byte));
        }

        todo!()
    }
}
