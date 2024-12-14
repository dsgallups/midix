//! Simple building-block data that can be read in one go.
//! All primitives have a known, fixed size.
//! Also, primitives advance the file pointer when read.

/// The order in which tracks should be laid out when playing back this SMF file.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Format {
    /// This file should have a single track only.
    ///
    /// If the `strict` feature is enabled, an error is raised if the format is
    /// `Format::SingleTrack` and there is not exactly one track.
    SingleTrack,
    /// This file has several tracks that should be played simultaneously.
    ///
    /// Usually the first track controls tempo and other song metadata.
    Parallel,
    /// This file has several tracks, each one a separate song.
    ///
    /// The tracks should be played sequentially, as completely separate MIDI tracks packaged
    /// within a single SMF file.
    Sequential,
}
impl Format {
    pub(crate) fn encode(&self) -> [u8; 2] {
        let code: u16 = match self {
            Format::SingleTrack => 0,
            Format::Parallel => 1,
            Format::Sequential => 2,
        };
        code.to_be_bytes()
    }
}

/// The timing for an SMF file.
/// This can be in ticks/beat or ticks/second.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Timing {
    /// Specifies ticks/beat as a 15-bit integer.
    ///
    /// The length of a beat is not standard, so in order to fully describe the length of a MIDI
    /// tick the [`MetaMessage::Tempo`](enum.MetaMessage.html#Tempo.v) event should be present.
    Metrical(u16),
    /// Specifies ticks/second by dividing a second into frames and then into subframes.
    /// Therefore the length of of a tick is `1/fps/subframe`.
    Timecode(Fps, u8),
}
impl Timing {
    pub(crate) fn encode(&self) -> [u8; 2] {
        match self {
            Timing::Metrical(ticksperbeat) => ticksperbeat.to_be_bytes(),
            Timing::Timecode(framespersec, ticksperframe) => {
                [(-(framespersec.as_int() as i8)) as u8, *ticksperframe]
            }
        }
    }
}

/// A timestamp encoding an SMPTE time of the day.
///
/// Enforces several guarantees:
///
/// - `hour` is inside [0, 23]
/// - `minute` is inside [0, 59]
/// - `second` is inside [0, 59]
/// - `frame` is inside [0, fps - 1]
/// - `subframe` is inside [0, 99]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct SmpteTime {
    hour: u8,
    minute: u8,
    second: u8,
    frame: u8,
    subframe: u8,
    fps: Fps,
}
impl SmpteTime {
    /// Create a new SMPTE timestamp with the given information.
    #[inline]
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
        frame: u8,
        subframe: u8,
        fps: Fps,
    ) -> Option<SmpteTime> {
        macro_rules! check {
            ($cond:expr) => {{
                if !{ $cond } {
                    return None;
                }
            }};
        }
        check!(hour < 24);
        check!(minute < 60);
        check!(second < 60);
        check!(frame < fps.as_int());
        check!(subframe < 100);
        Some(SmpteTime {
            hour,
            minute,
            second,
            frame,
            subframe,
            fps,
        })
    }

    /// Get the hour component of this timestamp.
    #[inline]
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute component of this timestamp.
    #[inline]
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Get the second component of this timestamp.
    #[inline]
    pub fn second(&self) -> u8 {
        self.second
    }

    /// Get the frame component of this timestamp.
    /// The meaning of this value depends on the value of `fps`.
    #[inline]
    pub fn frame(&self) -> u8 {
        self.frame
    }

    /// Get the subframe component of this timestamp (hundredths of a frame).
    #[inline]
    pub fn subframe(&self) -> u8 {
        self.subframe
    }

    /// Get the FPS component of this timestamp.
    #[inline]
    pub fn fps(&self) -> Fps {
        self.fps
    }

    /// Convert the second + frame + subframe components of this timestamp into a single
    /// floating-point number of seconds.
    /// Note that this does not include the hour and minute components.
    #[inline]
    pub fn second_f32(&self) -> f32 {
        self.second as f32
            + ((self.frame as f32 + self.subframe as f32 / 100.0) / self.fps.as_f32())
    }

    pub(crate) fn encode(&self) -> [u8; 5] {
        let hour_fps = self.hour() | self.fps().to_code() << 5;
        [
            hour_fps,
            self.minute(),
            self.second(),
            self.frame(),
            self.subframe(),
        ]
    }
}

/// One of the four FPS values available for SMPTE times, as defined by the MIDI standard.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Fps {
    /// 24 frames per second.
    Fps24,
    /// 25 frames per second.
    Fps25,
    /// Actually `29.97 = 30 / 1.001` frames per second.
    ///
    /// Quite an exotic value because of interesting historical reasons.
    Fps29,
    /// 30 frames per second.
    Fps30,
}
impl Fps {
    /// Does the conversion from a 2-bit fps code to an `Fps` value.
    pub(crate) fn from_code(code: u8) -> Option<Fps> {
        let v = match code {
            0 => Fps::Fps24,
            1 => Fps::Fps25,
            2 => Fps::Fps29,
            3 => Fps::Fps30,
            _ => return None,
        };
        Some(v)
    }

    /// Does the conversion to a 2-bit fps code.
    pub(crate) fn to_code(self) -> u8 {
        match self {
            Fps::Fps24 => 0,
            Fps::Fps25 => 1,
            Fps::Fps29 => 2,
            Fps::Fps30 => 3,
        }
    }

    /// Converts an integer representing the semantic fps to an `Fps` value (ie. `24` -> `Fps24`).
    #[inline]
    pub fn from_int(raw: u8) -> Option<Fps> {
        Some(match raw {
            24 => Fps::Fps24,
            25 => Fps::Fps25,
            29 => Fps::Fps29,
            30 => Fps::Fps30,
            _ => return None,
        })
    }

    /// Get the integral approximate fps out.
    #[inline]
    pub fn as_int(self) -> u8 {
        match self {
            Fps::Fps24 => 24,
            Fps::Fps25 => 25,
            Fps::Fps29 => 29,
            Fps::Fps30 => 30,
        }
    }

    /// Get the actual `f32` fps out.
    #[inline]
    pub fn as_f32(self) -> f32 {
        match self.as_int() {
            24 => 24.0,
            25 => 25.0,
            29 => 30.0 / 1.001,
            30 => 30.0,
            _ => unreachable!(),
        }
    }
}
impl From<Fps> for f32 {
    fn from(x: Fps) -> Self {
        x.as_f32()
    }
}
