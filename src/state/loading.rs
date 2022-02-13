use bevy::{asset::LoadState, prelude::*};

use crate::state::AppState;
use crate::sprite::SpriteHandles;

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::Loading)
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_character_textures))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_textures));
    }
}

fn load_character_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let mut load_impl = |name, start, number_of_frames| {
        let handles = (start..start+number_of_frames)
            .map(|i| format!["pixel-platformer/Characters/character_00{:0>2}.png", i])
            .map(|file| asset_server.load(file.as_str()))
            .collect();
        sprite_handles.handles.insert(name, handles);
    };
    load_impl("green", 0, 2);
    load_impl("blue", 2, 2);
    load_impl("pink", 4, 2);
    load_impl("yellow", 6, 2);
    load_impl("spike ball", 8, 1);
    load_impl("beige", 9, 2);
    load_impl("fish", 11, 2);
    load_impl("block", 13, 2);
    load_impl("hedgehog", 15, 3);
    load_impl("baby jeremy", 18, 3);
    load_impl("jeremy", 21, 3);
    load_impl("angel", 24, 3);
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let ids = sprite_handles.handles.iter()
        .map(|(_, handles)| handles.iter().map(|h| h.id))
        .flatten();
    if let LoadState::Loaded = asset_server.get_group_load_state(ids) {
        state.set(AppState::Game).unwrap();
    }
}