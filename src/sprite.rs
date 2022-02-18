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

pub enum SpriteVariant {
    Sprite(&'static str),
    SpriteSheet(HashMap<&'static str, &'static str>)
}

impl std::ops::Index<&'static str> for SpriteVariant {
    type Output = &'static str;
    fn index(&self, key: &'static str) -> &Self::Output {
        match self {
            SpriteVariant::Sprite(s) => s,
            SpriteVariant::SpriteSheet(map) => &map[key],
        }
    }
}

lazy_static!{
//TODO: maybe use enums instead of strings lol
pub static ref SPRITES: HashMap<&'static str, SpriteVariant> = HashMap::from([
    ("blue background", SpriteVariant::SpriteSheet(HashMap::from([
        ("empty", "pixel-platformer/Background/background_0000.png"),
        ("half", "pixel-platformer/Background/background_0001.png"),
        ("full", "pixel-platformer/Background/background_0002.png"),
    ]))),
    ("brown background", SpriteVariant::SpriteSheet(HashMap::from([
        ("empty", "pixel-platformer/Background/background_0003.png"),
        ("half", "pixel-platformer/Background/background_0004.png"),
        ("full", "pixel-platformer/Background/background_0005.png"),
    ]))),
    ("white background", SpriteVariant::SpriteSheet(HashMap::from([
        ("empty", "pixel-platformer/Background/background_0006.png"),
        ("half", "pixel-platformer/Background/background_0007.png"),
        ("full", "pixel-platformer/Background/background_0008.png"),
    ]))),
    ("green background", SpriteVariant::SpriteSheet(HashMap::from([
        ("empty", "pixel-platformer/Background/background_0009.png"),
        ("half", "pixel-platformer/Background/background_0010.png"),
        ("full", "pixel-platformer/Background/background_0011.png"),
    ]))),
    ("green idle", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0000.png"),
    ]))),
    ("green walk", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0000.png"),
        ("open", "pixel-platformer/Characters/character_0001.png"),
    ]))),
    ("green jump", SpriteVariant::SpriteSheet(HashMap::from([
        ("open", "pixel-platformer/Characters/character_0001.png"),
    ]))),
    ("blue", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0002.png"),
        ("open", "pixel-platformer/Characters/character_0003.png"),
    ]))),
    ("pink", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0004.png"),
        ("open", "pixel-platformer/Characters/character_0005.png"),
    ]))),
    ("yellow", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0006.png"),
        ("open", "pixel-platformer/Characters/character_0007.png"),
    ]))),
    ("spike ball", SpriteVariant::SpriteSheet(HashMap::from([
        ("idle", "pixel-platformer/Characters/character_0008.png"),
    ]))),
    ("beige", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0009.png"),
        ("open", "pixel-platformer/Characters/character_0010.png"),
    ]))),
    ("fish", SpriteVariant::SpriteSheet(HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0011.png"),
        ("open", "pixel-platformer/Characters/character_0012.png"),
    ]))),
    ("block", SpriteVariant::SpriteSheet(HashMap::from([
        ("surprised", "pixel-platformer/Characters/character_0013.png"),
        ("pissed", "pixel-platformer/Characters/character_0014.png"),
    ]))),
    ("hedgehog", SpriteVariant::SpriteSheet(HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0015.png"),
        ("idle1", "pixel-platformer/Characters/character_0016.png"),
        ("idle2", "pixel-platformer/Characters/character_0017.png"),
    ]))),
    ("baby jeremy", SpriteVariant::SpriteSheet(HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0018.png"),
        ("idle1", "pixel-platformer/Characters/character_0019.png"),
        ("idle2", "pixel-platformer/Characters/character_0020.png"),
    ]))),
    ("jeremy", SpriteVariant::SpriteSheet(HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0021.png"),
        ("idle1", "pixel-platformer/Characters/character_0022.png"),
        ("idle2", "pixel-platformer/Characters/character_0023.png"),
    ]))),
    ("angel", SpriteVariant::SpriteSheet(HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0024.png"),
        ("idle1", "pixel-platformer/Characters/character_0025.png"),
        ("idle2", "pixel-platformer/Characters/character_0026.png"),
    ]))),
    ("ground", SpriteVariant::SpriteSheet(HashMap::from([
        ("grass alone", "pixel-platformer/Tiles/tile_0000.png"),
        ("grass right", "pixel-platformer/Tiles/tile_0001.png"),
        ("grass left right", "pixel-platformer/Tiles/tile_0002.png"),
        ("grass left", "pixel-platformer/Tiles/tile_0003.png"),
        ("grass down", "pixel-platformer/Tiles/tile_0020.png"),
        ("grass down right", "pixel-platformer/Tiles/tile_0021.png"),
        ("grass down left right", "pixel-platformer/Tiles/tile_0022.png"),
        ("grass down left", "pixel-platformer/Tiles/tile_0023.png"),
        ("full", "pixel-platformer/Tiles/tile_0122.png"),
        ("above", "pixel-platformer/Tiles/tile_0140.png"),
        ("above left", "pixel-platformer/Tiles/tile_0143.png"),
        ("above right", "pixel-platformer/Tiles/tile_0141.png"),
        ("above below", "pixel-platformer/Tiles/tile_0120.png"),
        ("left empty", "pixel-platformer/Tiles/tile_0121.png"),
        ("right empty", "pixel-platformer/Tiles/tile_0123.png"),
        ("below empty", "pixel-platformer/Tiles/tile_0142.png"),
        ("below left empty", "pixel-platformer/Tiles/tile_0005.png"),
        ("below right empty", "pixel-platformer/Tiles/tile_0004.png"),
        ("above left empty", "pixel-platformer/Tiles/tile_0025.png"),
        ("above right empty", "pixel-platformer/Tiles/tile_0024.png"),
    ]))),
    ("heart", SpriteVariant::SpriteSheet(HashMap::from([
        ("full", "pixel-platformer/Tiles/tile_0044.png"),
        ("half", "pixel-platformer/Tiles/tile_0045.png"),
        ("empty", "pixel-platformer/Tiles/tile_0046.png"),
    ]))),
]);
}

pub fn load_sprites(name: &str, asset_server: &Res<AssetServer>) -> Vec<Handle<Image>> {
    match &SPRITES[name] {
        SpriteVariant::Sprite(path) => vec![asset_server.load(*path)],
        SpriteVariant::SpriteSheet(map) => map.iter().map(
            |(_, &path)| asset_server.load(path)
        ).collect(),
    }
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
