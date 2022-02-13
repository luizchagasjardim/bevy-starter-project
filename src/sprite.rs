use std::collections::HashMap;

use bevy::prelude::*;
use bevy::asset::HandleId;

#[derive(Default)]
pub struct SpriteHandles {
    pub handles: HashMap<&'static str, Vec<Handle<Image>>>,
}

impl SpriteHandles {
    pub fn id_list(&self) -> impl Iterator<Item = HandleId> + '_ {
        self.handles.iter()
            .map(|(_, handles)| handles.iter().map(|h| h.id))
            .flatten()
    }
}

#[derive(Component)]
pub struct SpriteTimer {
    pub timer: Timer,
}

impl SpriteTimer {
    pub fn from_seconds(duration: f32) -> Self {
        SpriteTimer {
            timer: Timer::from_seconds(duration, true)
        }
    }
}
