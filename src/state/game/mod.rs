use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

mod player;
use player::*;

mod velocity;
use velocity::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_background))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_characters))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animation))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(camera_movement));
    }
}

fn spawn_background(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let tile_size = 24.0;
    let background_layer = 0.0;
    let background_type = "blue background";
    let cloud_height = 3;
    let texture_atlas_handle = spawn("blue background", &sprite_handles, &mut texture_atlases, &mut textures);

    for i in -10..11 {
        for j in -10..11 {

            let image = if j < cloud_height { "full" } else if j == cloud_height { "half" } else { "empty" };
            let image = SPRITES[background_type][image];
            let handle = asset_server.get_handle(image);
            let texture_atlas = texture_atlases.get(texture_atlas_handle.clone()).unwrap();
            let sprite = TextureAtlasSprite {
                index: texture_atlas.get_texture_index(&handle).unwrap(),
                ..Default::default()
            };

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32*tile_size, j as f32*tile_size, background_layer)),
                    ..Default::default()
                });
        }
    }
}

fn spawn_characters(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut spawn = |name| {
        spawn(
            name,
            &sprite_handles,
            &mut texture_atlases,
            &mut textures,
        )
    };
    let character_size = 24.0;
    let player_layer = 2.0;
    let npc_layer = 1.0;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("green"),
            transform: Transform::from_translation(Vec3::new(-6.0*character_size, 0.0, player_layer)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2))
        .insert_bundle(PlayerBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("blue"),
            transform: Transform::from_translation(Vec3::new(-4.0*character_size, 0.0, npc_layer)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("pink"),
            transform: Transform::from_translation(Vec3::new(-2.0*character_size, 0.0, npc_layer)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("yellow"),
            transform: Transform::from_translation(Vec3::new(0.0*character_size, 0.0, npc_layer)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("jeremy"),
            transform: Transform::from_translation(Vec3::new(2.0*character_size, 0.0, npc_layer)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.5));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("block"),
            transform: Transform::from_translation(Vec3::new(4.0*character_size, 0.0, npc_layer)),
            ..Default::default()
        });
}

fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut SpriteTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut sprite_timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        sprite_timer.timer.tick(time.delta());
        if sprite_timer.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Controls, &mut Velocity)>,
) {
    for (controls, mut velocity) in query.iter_mut() {
        let mut direction = 0;
        if input.pressed(controls.left) {
            direction -= 1;
        }
        if input.pressed(controls.right) {
            direction += 1;
        }
        velocity.update(direction);
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn camera_movement(
    windows: Res<Windows>,
    player_query: Query<(&PlayerCharacter, &Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform), Without<PlayerCharacter>>,
) {
    let window = windows.get_primary().unwrap();
    let horizontal_limit = window.width() * 0.3;

    let (_, player_position) = player_query.single();
    let player_position = player_position.translation.x;

    let (_, mut camera_position) = camera_query.single_mut();

    let left_limit = camera_position.translation.x - horizontal_limit;
    let right_limit = camera_position.translation.x + horizontal_limit;

    if player_position < left_limit {
        camera_position.translation.x = player_position + horizontal_limit;
    } else if player_position > right_limit {
        camera_position.translation.x = player_position - horizontal_limit;
    }
}
