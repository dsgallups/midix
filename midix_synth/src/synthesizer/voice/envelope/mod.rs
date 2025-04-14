mod modulation;
pub(super) use modulation::*;

mod volume;
pub(super) use volume::*;

#[derive(Ord, PartialEq, PartialOrd, Eq, Clone, Copy, Debug)]
pub enum EnvelopeStage {
    Delay,
    Attack,
    Hold,
    Decay,
    Release,
}

impl EnvelopeStage {
    pub fn next(self) -> EnvelopeStage {
        use EnvelopeStage::*;
        match self {
            Delay => Attack,
            Attack => Hold,
            Hold => Decay,
            Decay => Release,
            Release => Release,
        }
    }
}
