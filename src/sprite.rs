use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use bevy::prelude::*;
use bevy::asset::HandleId;

#[derive(Default)]
pub struct SpriteHandles {
    pub handles: HashMap<String, Vec<Handle<Image>>>,
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

#[derive(Clone)]
pub enum SpriteVariant {
    Sprite(&'static str),
    SpriteSheet(SpriteType)
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SpriteType {
    BlueBG,
    BrownBG,
    WhiteBG,
    GreenBG,
    IdleGreen,
    WalkGreen,
    JumpGreen,
    Blue,
    Pink,
    Yellow,
    Beige,
    SpikeBall,
    Fish,
    Block,
    Hedgehog,
    BabyJeremy,
    Jeremy,
    Angel,
    Ground,
    Heart,
}

impl FromStr for SpriteType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BlueBG" => Ok(SpriteType::BlueBG),
            "BrownBG" => Ok(SpriteType::BrownBG),
            "WhiteBG" => Ok(SpriteType::WhiteBG),
            "GreenBG" => Ok(SpriteType::GreenBG),
            "IdleGreen" => Ok(SpriteType::IdleGreen),
            "WalkGreen" => Ok(SpriteType::WalkGreen),
            "JumpGreen" => Ok(SpriteType::JumpGreen),
            "Blue" => Ok(SpriteType::Blue),
            "Pink" => Ok(SpriteType::Pink),
            "Yellow" => Ok(SpriteType::Yellow),
            "Beige" => Ok(SpriteType::Beige),
            "SpikeBall" => Ok(SpriteType::SpikeBall),
            "Fish" => Ok(SpriteType::Fish),
            "Block" => Ok(SpriteType::Block),
            "Hedgehog" => Ok(SpriteType::Hedgehog),
            "BabyJeremy" => Ok(SpriteType::BabyJeremy),
            "Jeremy" => Ok(SpriteType::Jeremy),
            "Angel" => Ok(SpriteType::Angel),
            "Ground" => Ok(SpriteType::Ground),
            "Heart" => Ok(SpriteType::Heart),
            _ => Err(()),
        }
    }
}

impl fmt::Display for SpriteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum SpritePhysicalStates {
    Empty,
    Half,
    Full,
}

lazy_static!{
//TODO: maybe use enums instead of strings lol
pub static ref SPRITES: HashMap<SpriteType, HashMap<&'static str, &'static str>> = HashMap::from([
    (SpriteType::BlueBG, HashMap::from([
        ("empty", "pixel-platformer/Background/background_0000.png"),
        ("half", "pixel-platformer/Background/background_0001.png"),
        ("full", "pixel-platformer/Background/background_0002.png"),
    ])),
    (SpriteType::BrownBG, HashMap::from([
        ("empty", "pixel-platformer/Background/background_0003.png"),
        ("half", "pixel-platformer/Background/background_0004.png"),
        ("full", "pixel-platformer/Background/background_0005.png"),
    ])),
    (SpriteType::WhiteBG, HashMap::from([
        ("empty", "pixel-platformer/Background/background_0006.png"),
        ("half", "pixel-platformer/Background/background_0007.png"),
        ("full", "pixel-platformer/Background/background_0008.png"),
    ])),
    (SpriteType::GreenBG, HashMap::from([
        ("empty", "pixel-platformer/Background/background_0009.png"),
        ("half", "pixel-platformer/Background/background_0010.png"),
        ("full", "pixel-platformer/Background/background_0011.png"),
    ])),
    (SpriteType::IdleGreen, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0000.png"),
    ])),
    (SpriteType::WalkGreen, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0000.png"),
        ("open", "pixel-platformer/Characters/character_0001.png"),
    ])),
    (SpriteType::JumpGreen, HashMap::from([
        ("open", "pixel-platformer/Characters/character_0001.png"),
    ])),
    (SpriteType::Blue, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0002.png"),
        ("open", "pixel-platformer/Characters/character_0003.png"),
    ])),
    (SpriteType::Pink, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0004.png"),
        ("open", "pixel-platformer/Characters/character_0005.png"),
    ])),
    (SpriteType::Yellow, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0006.png"),
        ("open", "pixel-platformer/Characters/character_0007.png"),
    ])),
    (SpriteType::SpikeBall, HashMap::from([
        ("idle", "pixel-platformer/Characters/character_0008.png"),
    ])),
    (SpriteType::Beige, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0009.png"),
        ("open", "pixel-platformer/Characters/character_0010.png"),
    ])),
    (SpriteType::Fish, HashMap::from([
        ("closed", "pixel-platformer/Characters/character_0011.png"),
        ("open", "pixel-platformer/Characters/character_0012.png"),
    ])),
    (SpriteType::Block, HashMap::from([
        ("surprised", "pixel-platformer/Characters/character_0013.png"),
        ("pissed", "pixel-platformer/Characters/character_0014.png"),
    ])),
    (SpriteType::Hedgehog, HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0015.png"),
        ("idle1", "pixel-platformer/Characters/character_0016.png"),
        ("idle2", "pixel-platformer/Characters/character_0017.png"),
    ])),
    (SpriteType::BabyJeremy, HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0018.png"),
        ("idle1", "pixel-platformer/Characters/character_0019.png"),
        ("idle2", "pixel-platformer/Characters/character_0020.png"),
    ])),
    (SpriteType::Jeremy, HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0021.png"),
        ("idle1", "pixel-platformer/Characters/character_0022.png"),
        ("idle2", "pixel-platformer/Characters/character_0023.png"),
    ])),
    (SpriteType::Angel, HashMap::from([
        ("idle0", "pixel-platformer/Characters/character_0024.png"),
        ("idle1", "pixel-platformer/Characters/character_0025.png"),
        ("idle2", "pixel-platformer/Characters/character_0026.png"),
    ])),
    (SpriteType::Ground, HashMap::from([
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
    ])),
    (SpriteType::Heart, HashMap::from([
        ("full", "pixel-platformer/Tiles/tile_0044.png"),
        ("half", "pixel-platformer/Tiles/tile_0045.png"),
        ("empty", "pixel-platformer/Tiles/tile_0046.png"),
    ])),
]);
}

pub fn load_sprites(name: SpriteType, asset_server: &Res<AssetServer>) -> Vec<Handle<Image>> {
    SPRITES[&name].iter().map(
            |(_, &path)| asset_server.load(path)
        ).collect()
}

pub fn spawn(
    name: String,
    sprite_handles: &Res<SpriteHandles>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Image>>,
) -> Handle<TextureAtlas> {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.handles[&name].iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(textures).unwrap();
    texture_atlases.add(texture_atlas)
}
