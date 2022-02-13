use std::collections::HashMap;
use bevy::{asset::LoadState, prelude::*};

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_character_textures))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(animate_sprite_system))
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Game,
}

#[derive(Default)]
struct SpriteHandles {
    handles: HashMap<&'static str, Vec<Handle<Image>>>,
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

fn setup(
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
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("green"),
            transform: Transform::from_translation(Vec3::new(-144.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("blue"),
            transform: Transform::from_translation(Vec3::new(-96.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("pink"),
            transform: Transform::from_translation(Vec3::new(-48.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("yellow"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.2, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("jeremy"),
            transform: Transform::from_translation(Vec3::new(48.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.5, true));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("block"),
            transform: Transform::from_translation(Vec3::new(96.0, 0.0, 0.0)),
            ..Default::default()
        });
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}