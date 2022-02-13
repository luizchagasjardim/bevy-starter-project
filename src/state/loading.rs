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
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_textures))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(remove_loading_bar));
    }
}

fn load_preloaded_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let handles = SPRITES["heart"].iter().map(
        |(_, &path)| asset_server.load(path)
    ).collect();
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

impl LoadingBar {
    fn get_image(&self, percent: f32) -> &str {
        if percent < self.lower_bound {
            SPRITES["heart"]["empty"]
        } else if percent >= self.upper_bound {
            SPRITES["heart"]["full"]
        } else {
            SPRITES["heart"]["half"]
        }
    }
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

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    let mut load = |path, name, start, number_of_frames| {
        let handles = (start..start+number_of_frames)
            .map(|i| asset_server.load(format!["{}_{:0>4}.png", path, i].as_str()))
            .collect();
        sprite_handles.handles.insert(name, handles);
    };
    let mut load_character = |name, start, number_of_frames| {
        load("pixel-platformer/Characters/character", name, start, number_of_frames);
    };
    load_character("green", 0, 2);
    load_character("blue", 2, 2);
    load_character("pink", 4, 2);
    load_character("yellow", 6, 2);
    load_character("spike ball", 8, 1);
    load_character("beige", 9, 2);
    load_character("fish", 11, 2);
    load_character("block", 13, 2);
    load_character("hedgehog", 15, 3);
    load_character("baby jeremy", 18, 3);
    load_character("jeremy", 21, 3);
    load_character("angel", 24, 3);
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&LoadingBar, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
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
    for (loading_bar, mut sprite, texture_atlas_handle) in query.iter_mut() {
        let image = loading_bar.get_image(percent);
        let handle = asset_server.get_handle(image);
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = texture_atlas.get_texture_index(&handle).unwrap();
    }
}

fn remove_loading_bar(
    mut commands: Commands,
    query: Query<(Entity, &LoadingBar)>,
) {
    for (id, _) in query.iter() {
        commands.entity(id).despawn();
    }
}
