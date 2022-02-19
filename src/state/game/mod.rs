use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

mod direction;

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
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(update_direction))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(player_spritesheet))
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
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    for tile_info in read_map().tile_info_iter() {
        if let Some(tile_info) = tile_info {
            let mut entity = match tile_info.image {
                SpriteVariant::Sprite(path) => commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.get_handle(path),
                        transform: Transform::from_translation(tile_info.position),
                        ..Default::default()
                    }),
                SpriteVariant::SpriteSheet(map) => commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: spawn(tile_info.image_key, &sprite_handles, &mut texture_atlases, &mut textures),
                        transform: Transform::from_translation(tile_info.position),
                        ..Default::default()
                    }),
            };
            if let Some(hitbox) = tile_info.hitbox {
                match tile_info.tile_type {
                    Tile::Empty => panic!("Not possible to have a hitbox on an empty tile"),
                    Tile::Ground => { entity.insert(GroundHitbox(hitbox)); },
                    Tile::Player => {
                        entity.insert(SpriteTimer::from_seconds(0.2))
                        .insert_bundle(PlayerBundle {
                            ground_hitbox: PlayerGroundHitbox(hitbox.clone()),
                            enemy_hitbox: PlayerEnemyHitbox(hitbox),
                            ..Default::default()
                        });
                    },
                }
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
    let character_size = 18.0;
    let npc_layer = 1.0;

    /*
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("green idle"),
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
    */
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

fn update_direction(mut query: Query<(&mut TextureAtlasSprite, &direction::Direction)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = *direction == direction::Direction::Right;
    }
}

fn player_spritesheet(
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut query: Query<(&mut PlayerCharacter, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>,
) {
    for (mut player, mut sprite, mut texture_atlas_handle) in query.iter_mut() {
        if let Some(sheet) = player.update_spritesheet() {
            *texture_atlas_handle = spawn(sheet, &sprite_handles, &mut texture_atlases, &mut textures);
            *sprite = TextureAtlasSprite::default();
        }
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerCharacter, &Controls, &mut Velocity, &mut direction::Direction)>,
) {
    for (mut player, controls, mut velocity, mut direction) in query.iter_mut() {
        let new_direction = direction::Direction::from_input(input.pressed(controls.left), input.pressed(controls.right));
        velocity.update(new_direction);
        if let Some(new_direction) = new_direction {
            *direction = new_direction;
        }
        player.update_walk_state(velocity.0.x);

        if input.just_pressed(controls.jump) {
            if let Ok(_) = player.try_jump() {
                velocity.0.y = 500.0;
            }
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
    mut player_query: Query<(&mut PlayerCharacter, &PlayerGroundHitbox, &mut Transform, &mut Velocity), Without<GroundHitbox>>,
) {
    for (mut player, player_hitbox, mut player_transform, mut player_velocity) in player_query.iter_mut() {
        for (ground_hitbox, ground_transform) in ground_query.iter() {
            if let Some(collision) = player_hitbox.0.collide(&player_transform.translation, &ground_hitbox.0, &ground_transform.translation) {
                match collision.collision_type {
                    CollisionType::Bottom => {
                        player_transform.translation.y += collision.overlap;
                        if player_velocity.0.y < 0.0 {
                            player_velocity.0.y = 0.0;
                            player.hit_ground();
                        }
                    },
                    CollisionType::Top => {
                        player_transform.translation.y -= collision.overlap;
                        player_velocity.stop_top();
                    },
                    CollisionType::Left => {
                        player_transform.translation.x += collision.overlap;
                        player_velocity.stop_left();
                    },
                    CollisionType::Right => {
                        player_transform.translation.x -= collision.overlap;
                        player_velocity.stop_right();
                    },
                };
            }
        }
    }
}
