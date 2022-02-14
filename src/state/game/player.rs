use bevy::prelude::*;
use super::velocity::Velocity;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player_character: PlayerCharacter,
    pub controls: Controls,
    pub velocity: Velocity,
}

#[derive(Component, Default)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Controls {
    pub left: KeyCode,
    pub right: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            left: KeyCode::A,
            right: KeyCode::D,
        }
    }
}