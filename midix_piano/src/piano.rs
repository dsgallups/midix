use bevy::{prelude::*, utils::HashMap};
use bevy_midix::prelude::*;

#[derive(Resource, Clone)]
pub struct Piano {
    keys: HashMap<Key<'static>, bool>,
}

impl Default for Piano {
    fn default() -> Self {
        let mut keys = HashMap::with_capacity(128);

        for key in Key::all() {
            keys.insert(key, false);
        }

        Self { keys }
    }
}
