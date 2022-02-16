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

#[derive(Clone, Copy, Default, PartialEq)]
pub enum PlayerState {
    #[default]
    IDLE,
    WALKING,
    JUMP,
    FALLING
}

#[derive(Component, Default)]
pub struct PlayerCharacter {
    state: PlayerState
}

impl PlayerCharacter {
    pub fn update_state_by_input(&mut self, walking: bool, tried_to_jump: bool) -> PlayerState {
        self.state = match &self.state {
            PlayerState::IDLE => if tried_to_jump { PlayerState::JUMP } else if walking { PlayerState::WALKING } else { PlayerState::IDLE },
            PlayerState::WALKING => if tried_to_jump { PlayerState::JUMP } else { PlayerState::WALKING },
            PlayerState::JUMP => PlayerState::FALLING,
            PlayerState::FALLING => PlayerState::FALLING,
        };
        self.state
    }
    pub fn jump(&mut self) {
        self.state = PlayerState::FALLING;
    }
    pub fn hit_ground(&mut self) {
        if self.state == PlayerState::FALLING {
            self.state = PlayerState::IDLE; //TODO: include walking
        }
    }
    pub fn hit_wall(&mut self) {
        if self.state == PlayerState::WALKING {
            self.state = PlayerState::IDLE;
        }
    }
}

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
