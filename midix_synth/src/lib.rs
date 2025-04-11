pub mod reader;
pub mod soundfont;
pub mod synthesizer;

pub mod prelude {
    pub use crate::reader::*;
    pub use crate::soundfont::*;
    pub use crate::soundfont::{generator::*, instrument::*, preset::*, zone::*};
    pub use crate::synthesizer::*;
}
