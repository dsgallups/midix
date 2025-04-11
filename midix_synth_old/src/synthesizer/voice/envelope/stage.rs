#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub struct EnvelopeStage {}

#[allow(unused)]
impl EnvelopeStage {
    pub const DELAY: i32 = 0;
    pub const ATTACK: i32 = 1;
    pub const HOLD: i32 = 2;
    pub const DECAY: i32 = 3;
    pub const RELEASE: i32 = 4;
}
