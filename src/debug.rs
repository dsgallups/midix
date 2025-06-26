#![doc = r#"
Internal debugging utilities
"#]
use crate::prelude::*;

/// Implements traits to handle debugging to stdout
pub struct DebugTarget<T> {
    inner: T,
}

impl<T: MidiTarget> MidiTarget for DebugTarget<T> {
    type Error = T::Error;
    fn handle_event(&mut self, event: MidiMessage) -> Result<(), Self::Error> {
        println!("Handling {:?}", event);
        self.inner.handle_event(event)
    }
}
