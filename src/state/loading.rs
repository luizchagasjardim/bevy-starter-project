use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app
            .add_state(AppState::PreLoad)
            .add_system_set(SystemSet::on_enter(AppState::PreLoad).with_system(load_preloaded_textures))
            .add_system_set(SystemSet::on_update(AppState::PreLoad).with_system(check_preloaded_textures))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_loading_bar))
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_character_textures))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_textures))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(remove_loading_bar));
    }
}

fn load_preloaded_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let handles = vec![
        asset_server.load("pixel-platformer/Tiles/tile_0044.png"),
        asset_server.load("pixel-platformer/Tiles/tile_0045.png"),
        asset_server.load("pixel-platformer/Tiles/tile_0046.png"),
    ];
    sprite_handles.handles.insert("loading", handles);
}

fn check_preloaded_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let ids = sprite_handles.id_list();
    if let LoadState::Loaded = asset_server.get_group_load_state(ids) {
        state.set(AppState::Loading).unwrap();
    }
}

#[derive(Component)]
struct LoadingBar {
    lower_bound: f32,
    upper_bound: f32,
}

pub fn setup_loading_bar(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut spawn = |name| {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in sprite_handles.handles[name].iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone(), texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        texture_atlases.add(texture_atlas)
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let n = 10;
    for i in 0..n {
        let position = Vec3::new(-120.0 + 24.0 * (i as f32), -144.0, 0.0);
        let interval = 100.0 / n as f32;
        let lower_bound = i as f32 * interval;
        let upper_bound = lower_bound + interval;
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: spawn("loading"),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(LoadingBar { lower_bound, upper_bound });
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
    mut query: Query<(&LoadingBar, &mut TextureAtlasSprite)>,
) {
    let ids = sprite_handles.id_list();
    let mut total = 0;
    let mut loaded = 0;
    for id in ids {
        total += 1;
        match asset_server.get_load_state(id) {
            LoadState::Loaded => loaded += 1,
            LoadState::Failed => panic!("Failed to load asset."),
            _ => {},
        }
    }
    if loaded == total {
        state.set(AppState::Game).unwrap();
        return;
    }
    let percent = (100*loaded) as f32 / total as f32;
    for (loading_bar, mut sprite) in query.iter_mut() {
        //TODO: these hardcoded values are probably not guaranteed to work
        if percent < loading_bar.lower_bound {
            sprite.index = 2;
        } else if percent > loading_bar.upper_bound {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }
    }
}

fn remove_loading_bar(
    mut commands: Commands,
    mut query: Query<(Entity, &LoadingBar)>,
) {
    for (id, _) in query.iter() {
        commands.entity(id).despawn();
    }
}
