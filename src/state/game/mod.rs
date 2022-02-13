use bevy::prelude::*;

use crate::state::AppState;
use crate::sprite::*;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(animate_sprite_system));
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
        .insert(SpriteTimer::from_seconds(0.2));
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

fn animate_sprite_system(
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
