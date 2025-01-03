#![allow(dead_code)]

#[allow(unused)]
#[non_exhaustive]
pub struct LoopMode {}

#[allow(unused)]
impl LoopMode {
    pub const NO_LOOP: i32 = 0;
    pub const CONTINUOUS: i32 = 0;
    pub const LOOP_UNTIL_NOTE_OFF: i32 = 0;
}
