/// The possible FPS for MIDI tracks and files
///
/// the MIDI spec defines only four possible frame types:
/// - 24: 24fps
/// - 25: 25fps
/// - 29: dropframe 30 (30,000 frames / 1001 seconds)
/// - 30: 30fps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmpteFps {
    /// 24
    TwentyFour,
    /// 25
    TwentyFive,
    /// Note this is actually 29.997
    TwentyNine,
    /// 30
    Thirty,
}

impl SmpteFps {
    /// Most likely want to use this.
    /// Drop 30 (TwentyNine) is 30 here.
    pub fn as_division(&self) -> u8 {
        match self {
            Self::TwentyFour => 24,
            Self::TwentyFive => 25,
            Self::TwentyNine => 30,
            Self::Thirty => 30,
        }
    }
    /// Get the actual number of frames per second
    ///
    /// This is useful since I'm not interested in
    /// skipping frames 0 and 1 every minute that's not a multiple of 10.
    ///
    /// However, that's not to say this logic isn't faulty. If it is,
    /// please file an issue.
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::TwentyFour => 24.,
            Self::TwentyFive => 25.,
            Self::TwentyNine => DROP_FRAME,
            Self::Thirty => 30.,
        }
    }
}
const DROP_FRAME: f64 = 30_000. / 1001.;
