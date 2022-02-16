use bevy::prelude::*;
use super::hitbox::{PlayerGroundHitbox, PlayerEnemyHitbox};
use super::velocity::Velocity;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player_character: PlayerCharacter,
    pub controls: Controls,
    pub ground_hitbox: PlayerGroundHitbox,
    pub enemy_hitbox: PlayerEnemyHitbox,
    pub velocity: Velocity,
}

#[derive(Component, Default)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
        }
    }
}
