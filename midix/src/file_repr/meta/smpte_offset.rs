use crate::{SmpteError, prelude::SmpteFps};

/// A representation of a track's offset from the beginning of a midi file.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SmpteOffset {
    /// the track's fps. Note: this should be identical to a file's FPS if
    /// the file is defined in terms of `smpte`
    pub fps: SmpteFps,
    /// the hour offset. Should be between 0-23
    pub hour: u8,
    /// the minute offset. Should be between 0-59
    pub minute: u8,
    /// the second offset. Should be between 0-59
    pub second: u8,
    /// the offset within the second.
    /// note that frames start at 0.
    /// This is the frame within the second.
    pub frame: u8,
    /// the subframe offset. Should be between 0-99
    pub subframe: u8,
}

impl SmpteOffset {
    /// Override this value's provided fps. Used when a file is defined in smpte
    pub const fn as_micros_with_override(&self, fps: SmpteFps) -> f64 {
        ((((self.hour as u32 * 3600) + (self.minute as u32) * 60 + self.second as u32) * 1_000_000)
            as f64)
            + ((self.frame as u32) * 1_000_000) as f64 / fps.as_f64()
            + ((self.subframe as u32) * 10_000) as f64 / fps.as_f64()
    }
    /// Get the offset in terms of microseconds
    pub const fn as_micros(&self) -> f64 {
        ((((self.hour as u32 * 3600) + (self.minute as u32) * 60 + self.second as u32) * 1_000_000)
            as f64)
            + ((self.frame as u32) * 1_000_000) as f64 / self.fps.as_f64()
            + ((self.subframe as u32) * 10_000) as f64 / self.fps.as_f64()
    }

    /// Parse the offset given some slice with a length of 5
    pub const fn parse(data: &[u8]) -> Result<Self, SmpteError> {
        if data.len() != 5 {
            return Err(SmpteError::Length(data.len()));
        }

        // 0 rr hhhhh
        let frame_type = match data[0] >> 5 {
            0 => SmpteFps::TwentyFour,
            1 => SmpteFps::TwentyFive,
            2 => SmpteFps::TwentyNine,
            3 => SmpteFps::Thirty,
            v => return Err(SmpteError::TrackFrame(v)),
        };
        let hour = data[0] & 0b0001_1111;
        if hour > 24 {
            return Err(SmpteError::HourOffset(hour));
        }
        let minute = data[1];
        if minute > 59 {
            return Err(SmpteError::MinuteOffset(minute));
        }
        let second = data[2];
        if second > 59 {
            return Err(SmpteError::SecondOffset(second));
        }

        let frame = data[3];
        // always 1/100 of frame
        let subframe = data[4];
        if subframe > 99 {
            return Err(SmpteError::Subframe(subframe));
        }
        Ok(Self {
            fps: frame_type,
            hour,
            minute,
            second,
            frame,
            subframe,
        })
    }
}

#[test]
fn parse_smpte_offset() {
    use pretty_assertions::assert_eq;
    // this are the bytes after 00 FF 54 05
    // where 54 is smpte offset, and 05 is length five.
    let bytes = [0x41, 0x17, 0x2D, 0x0C, 0x22];
    let offset = SmpteOffset::parse(&bytes).unwrap();

    assert_eq!(offset.fps, SmpteFps::TwentyNine);
    assert_eq!(offset.hour, 1);
    assert_eq!(offset.minute, 23);
    assert_eq!(offset.second, 45);
    assert_eq!(offset.frame, 12);
    assert_eq!(offset.subframe, 34);
}

#[test]
fn parse_invalid_smpte_offset() {
    use pretty_assertions::assert_eq;
    // this are the bytes after 00 FF 54 05
    // where 54 is smpte offset, and 05 is length five.
    let bytes = [0x7F, 0x17, 0x2D, 0x0C, 0x22];
    let err = SmpteOffset::parse(&bytes).unwrap_err();
    assert_eq!(err, SmpteError::HourOffset(31));

    let bytes = [0x41, 0x50, 0x2D, 0x0C, 0x22];
    let err = SmpteOffset::parse(&bytes).unwrap_err();
    assert_eq!(err, SmpteError::MinuteOffset(80));
}
