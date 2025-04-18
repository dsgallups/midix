#![doc = r#"
Components to make songs programatically
"#]

mod channel_settings;
pub use channel_settings::*;

mod beat;
pub use beat::*;

mod simple_song;
pub use simple_song::*;

mod section;
pub use section::*;

mod bad_idea;
pub use bad_idea::*;
