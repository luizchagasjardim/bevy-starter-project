use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

mod hitbox;
use hitbox::*;

mod map;
use map::*;

mod player;
use player::*;

mod velocity;
use velocity::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_background))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_map))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_characters))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animation))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_ground_collision))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(camera_movement));
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = 24.0;
    let layer = 0.0;
    for i in -10..11 {
        for j in -10..11 {
            //TODO: use one big image looping or just moving with the camera instead of creating a trillion entities
            let background_type = "blue background";
            let cloud_height = 3;
            let image = if j < cloud_height { "full" } else if j == cloud_height { "half" } else { "empty" };
            let image = SPRITES[background_type][image];
            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.get_handle(image),
                    transform: Transform::from_translation(Vec3::new(i as f32*tile_size, j as f32*tile_size, layer)),
                    ..Default::default()
                });
        }
    }
}

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let map = read_map();
    let tile_size = 18.0;
    let start_point = -tile_size * (map.len()/2) as f32;
    let layer = 0.5;
    let ground_type = "ground";
    for (i, line) in map.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            let image = match tile {
                Tile::EMPTY => None,
                Tile::GROUND => {
                    //TODO: methods for getting the neighbours
                    let left = if i > 0 { map[i-1][j] } else { Tile::EMPTY };
                    let right = if i+1 < map.len() { map[i+1][j] } else { Tile::EMPTY };
                    let below = if j > 0 { map[i][j-1] } else { Tile::EMPTY };
                    let above = if j+1 < line.len() { map[i][j+1] } else { Tile::EMPTY };
                    let below_left = if i > 0 && j > 0 { map[i-1][j-1] } else { Tile::EMPTY };
                    let below_right = if i+1 < line.len() && j > 0 { map[i+1][j-1] } else { Tile::EMPTY };
                    let above_left = if i > 0 && j+1 < line.len() { map[i-1][j+1] } else { Tile::EMPTY };
                    let above_right = if i+1 < line.len() && j+1 < line.len() { map[i+1][j+1] } else { Tile::EMPTY };
                    let image = match above {
                        Tile::EMPTY => match (left, right, below) {
                            //TODO: instead of checking for empty, create a method that return true if the neighbours connect
                            (Tile::EMPTY, Tile::EMPTY, Tile::EMPTY) => "grass alone",
                            (_, Tile::EMPTY, Tile::EMPTY) => "grass left",
                            (Tile::EMPTY, _, Tile::EMPTY) => "grass right",
                            (_, _, Tile::EMPTY) => "grass left right",
                            (Tile::EMPTY, Tile::EMPTY, _) => "grass down",
                            (_, Tile::EMPTY, _) => "grass down left",
                            (Tile::EMPTY, _, _) => "grass down right",
                            (_, _, _) => "grass down left right",
                        }
                        Tile::GROUND => match (left, right, below) {
                            (Tile::EMPTY, Tile::EMPTY, Tile::EMPTY) => "above",
                            (_, Tile::EMPTY, Tile::EMPTY) => "above left",
                            (Tile::EMPTY, _, Tile::EMPTY) => "above right",
                            (_, _, Tile::EMPTY) => "below empty",
                            (Tile::EMPTY, Tile::EMPTY, _) => "above below",
                            (_, Tile::EMPTY, _) => "right empty",
                            (Tile::EMPTY, _, _) => "left empty",
                            (_, _, _) => match (below_left, below_right, above_left, above_right) {
                                //TODO: missing some cases due to not having the images
                                (Tile::EMPTY, _, _, _) => "below left empty",
                                (_, Tile::EMPTY, _, _) => "below right empty",
                                (_, _, Tile::EMPTY, _) => "above left empty",
                                (_, _, _, Tile::EMPTY) => "above right empty",
                                (_, _, _, _) => "full",
                            }
                        }
                    };
                    Some(SPRITES[ground_type][image])
                }
            };
            if let Some(image) = image {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.get_handle(image),
                        transform: Transform::from_translation(Vec3::new(
                            start_point + i as f32 * tile_size,
                            start_point + j as f32 * tile_size,
                            layer,
                        )),
                        ..Default::default()
                    })
                    .insert(GroundHitbox(Hitbox {
                        relative_position: Vec3::default(),
                        size: Vec2::new(tile_size, tile_size),
                    }));
            }
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
        .insert_bundle(PlayerBundle {
            ground_hitbox: PlayerGroundHitbox(Hitbox {
                relative_position: Vec3::default(),
                size: Vec2::new(character_size, character_size), //TODO: better values
            }),
            enemy_hitbox: PlayerEnemyHitbox(Hitbox {
                relative_position: Vec3::default(),
                size: Vec2::new(character_size, character_size), //TODO: better values
            }),
            ..Default::default()
        });
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
        if input.pressed(controls.jump) {
            velocity.0.y = 150.0;
        }
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.apply_gravity(time.delta_seconds());
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

fn player_ground_collision(
    ground_query: Query<(&GroundHitbox, &Transform), Without<PlayerGroundHitbox>>,
    mut player_query: Query<(&PlayerGroundHitbox, &mut Transform, &mut Velocity), Without<GroundHitbox>>,
) {
    for (player_hitbox, mut player_transform, mut player_velocity) in player_query.iter_mut() {
        for (ground_hitbox, ground_transform) in ground_query.iter() {
            if let Some(collision) = player_hitbox.0.collide(&player_transform.translation, &ground_hitbox.0, &ground_transform.translation) {
                match collision.collision_type {
                    CollisionType::Bottom => {
                        player_transform.translation.y += collision.overlap;
                        player_velocity.stop_bottom();
                    },
                    CollisionType::Top => {
                        player_transform.translation.y -= collision.overlap;
                        player_velocity.stop_top()
                    },
                    CollisionType::Left => {
                        player_transform.translation.x += collision.overlap;
                        player_velocity.stop_left()
                    },
                    CollisionType::Right => {
                        player_transform.translation.x -= collision.overlap;
                        player_velocity.stop_right()
                    },
                };
            }
        }
    }
}
