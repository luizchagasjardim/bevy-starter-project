use std::collections::HashMap;

use bevy::prelude::*;
use bevy::asset::HandleId;

use phf::phf_map;

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

pub const SPRITES: phf::Map<&'static str, &'static str> = phf_map! {
    "full heart" => "pixel-platformer/Tiles/tile_0044.png",
    "half heart" => "pixel-platformer/Tiles/tile_0045.png",
    "empty heart" => "pixel-platformer/Tiles/tile_0046.png",
};