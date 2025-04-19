#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmpteFrame {
    TwentyFour,
    TwentyFive,
    /// Note this is actually 29.997
    TwentyNine,
    Thirty,
}
