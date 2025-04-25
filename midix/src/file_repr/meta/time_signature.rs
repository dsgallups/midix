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
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TimeSignature([u8; 4]);

impl Default for TimeSignature {
    fn default() -> Self {
        Self([4, 2, 24, 32])
    }
}

impl TimeSignature {
    /// 4 bytes; 6/8 time; 24 MIDI clocks/click, 8 32nd notes/ 24 MIDI clocks (24 MIDI clocks = 1 crotchet = 1 beat)
    /// is represented by
    /// ```rust
    /// # use midix::prelude::*;
    /// let x = TimeSignature::new_from_parts(6, 8, 24, 8);
    /// # assert_eq!(x.den(), 3);
    /// ```
    #[cfg(feature = "std")]
    pub fn new_from_parts(
        numerator: u8,
        denominator: u8,
        clocks_per_quarter: u8,
        notes_per_clocks: u8,
    ) -> Self {
        let pow_den = f64::log2(denominator as f64) as u8;

        Self([numerator, pow_den, clocks_per_quarter, notes_per_clocks])
    }

    /// Interpret a byte slice as a time signature
    pub const fn new_from_bytes(v: [u8; 4]) -> Self {
        Self(v)
    }
    /// numerator of the time signature
    pub const fn num(&self) -> u8 {
        self.0[0]
    }
    /// A negative power of two.
    ///
    /// if this returns 3, then 2^3 = 8, so it's representative of an eigth
    pub const fn den(&self) -> u8 {
        self.0[1]
    }
    /// midi clocks in a metronome click
    pub const fn clocks_per_click(&self) -> u8 {
        self.0[2]
    }

    ///(24 MIDI clocks = 1 crotchet = 1 beat). 24 midi clocks is a MIDI quarter note
    pub const fn notated_32nds_per_24_clocks(&self) -> u8 {
        self.0[3]
    }
}
