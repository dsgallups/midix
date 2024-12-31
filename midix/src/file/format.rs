use super::track::Track;
/*
TODO: Use this for the MIDI file type.

FF 00 02 Sequence Number
This optional event, which must occur at the beginning of a track,
before any nonzero delta-times, and before any transmittable MIDI
events, specifies the number of a sequence. In a format 2 MIDI File,
it is used to identify each "pattern" so that a "song" sequence using
the Cue message can refer to the patterns. If the ID numbers are
omitted, the sequences' locations in order in the file are used as
defaults. In a format 0 or 1 MIDI File, which only contain one
sequence, this number should be contained in the first (or only)
track. If transfer of several multitrack sequences is required,
this must be done as a group of format 1 files, each with a different
sequence number.*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format<'a> {
    /// Format 0
    SingleMultiChannel(Track<'a>),
    /// Format 1
    Simultaneous(Vec<Track<'a>>),
    /// Format 2
    SequentiallyIndependent(Vec<Track<'a>>),
}
