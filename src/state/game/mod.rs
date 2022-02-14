use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

mod controls;
use controls::*;

mod velocity;
use velocity::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animation))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(input))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(movement));
    }
}

fn setup(
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("green"),
            transform: Transform::from_translation(Vec3::new(-144.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2))
        .insert(Velocity::default())
        .insert(Controls::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("blue"),
            transform: Transform::from_translation(Vec3::new(-96.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("pink"),
            transform: Transform::from_translation(Vec3::new(-48.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("yellow"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.2));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("jeremy"),
            transform: Transform::from_translation(Vec3::new(48.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(SpriteTimer::from_seconds(0.5));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: spawn("block"),
            transform: Transform::from_translation(Vec3::new(96.0, 0.0, 0.0)),
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
