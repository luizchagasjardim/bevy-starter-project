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

lazy_static!{
pub static ref SPRITES: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::from([
    ("heart", HashMap::from([
        ("full", "pixel-platformer/Tiles/tile_0044.png"),
        ("half", "pixel-platformer/Tiles/tile_0045.png"),
        ("empty", "pixel-platformer/Tiles/tile_0046.png"),
    ])),
]);
}

pub fn load_sprites(name: &str, asset_server: Res<AssetServer>) -> Vec<Handle<Image>> {
    SPRITES[name].iter().map(
        |(_, &path)| asset_server.load(path)
    ).collect()
}

pub fn spawn(
    name: &str,
    sprite_handles: &Res<SpriteHandles>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Image>>,
) -> Handle<TextureAtlas> {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles[name].iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(textures).unwrap();
    texture_atlases.add(texture_atlas)
}
