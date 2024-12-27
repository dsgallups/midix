use std::ops::Neg;

mod message;
pub use message::*;

/// (in microseconds per MIDI quarter-note)
///
/// FF 51 03 tttttt
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TempoRef<'a>(&'a [u8; 3]);

impl<'a> TempoRef<'a> {
    pub fn new(v: &'a [u8; 3]) -> Self {
        Self(v)
    }
}

#[doc = r#"
FF 58 04 nn dd cc bb Time Signature

The time signature is expressed as four numbers.
nn and dd represent the numerator and denominator of the
time signature as it would be notated. The denominator is
a negative power of two: 2 represents a quarter-note,
3 represents an eighth-note, etc.
The cc parameter expresses the number of MIDI clocks in a
metronome click.
The bb parameter expresses the number of notated 32nd-notes
in a MIDI quarter-note (24 MIDI clocks).
This was added because there are already multiple
programs which allow a user to specify that what MIDI thinks of as
a quarter-note (24 clocks) is to be notated as, or related to in terms
of, something else.

Therefore, the complete event for 6/8 time, where the metronome clicks
every three eighth-notes, but there are 24 clocks per quarter-note, 72
to the bar, would be (in hex):

FF 58 04 06 03 24 08

That is, 6/8 time (8 is 2 to the 3rd power, so this is 06 03),
36 MIDI clocks per dotted-quarter (24 hex!), and
eight notated 32nd-notes per quarter-note.
"#]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct TimeSignatureRef<'a>(&'a [u8; 4]);

impl<'a> TimeSignatureRef<'a> {
    pub fn new(v: &'a [u8; 4]) -> Self {
        Self(v)
    }
    /// numerator of the time signature
    pub fn num(&self) -> u8 {
        self.0[0]
    }
    /// A negative power of two.
    ///
    /// if this returns 3, then 2^3 = 8, so it's representative of an eigth
    pub fn den(&self) -> u8 {
        self.0[1]
    }
    /// midi clocks in a metronome click
    pub fn clocks_per_click(&self) -> u8 {
        self.0[2]
    }
    pub fn notated_32nds_in_midi_quarter_note(&self) -> u8 {
        self.0[3]
    }
}

#[doc = r#"
FF 59 02 sf mi Key Signature
sf = -7: 7 flats
sf = -1: 1 flat
sf = 0: key of C
sf = 1: 1 sharp
sf = 7: 7 sharps

mi = 0: major key
mi = 1: minor key
"#]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct KeySignatureRef<'a>(&'a [u8; 2]);
impl<'a> KeySignatureRef<'a> {
    pub fn new(v: &'a [u8; 2]) -> Self {
        Self(v)
    }
    pub fn sharp_flat_count(&self) -> i8 {
        self.0[0] as i8
    }

    pub fn num_sharps(&self) -> u8 {
        self.sharp_flat_count().min(0).unsigned_abs()
    }
    pub fn num_flats(&self) -> u8 {
        self.sharp_flat_count().neg().min(0).unsigned_abs()
    }
    pub fn minor_key(&self) -> bool {
        self.0[1] == 1
    }
}
