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
enum PlayerState {
    #[default]
    Idle,
    Walking,
    InTheAir(u8),
}

#[derive(Component, Default)]
pub struct PlayerCharacter {
    state: PlayerState,
}

impl PlayerCharacter {
    const MAX_JUMP_COUNT: u8 = 2;
    pub fn get_jump_count(&self) -> u8 {
        if let PlayerState::InTheAir(jump_count) = self.state {
            jump_count
        } else {
            0
        }
    }
    pub fn update_by_walk_input(&mut self, walking: bool) {}
    pub fn try_jump(&mut self) -> Result<(), ()> {
        let jump_count = self.get_jump_count();
        if jump_count < Self::MAX_JUMP_COUNT {
            self.state = PlayerState::InTheAir(jump_count+1);
            Ok(())
        } else {
            crate::log::log("Reached jump limit!"); //TODO: if this function does not have a log somewhere, the code behaves as if MAX_JUMP_COUNT is one more. Good luck.
            Err(())
        }
    }
    pub fn hit_ground(&mut self) {
        if let PlayerState::InTheAir(_) = self.state {
            self.state = PlayerState::Idle; //TODO: include walking
        }
    }
    pub fn hit_wall(&mut self) {
        if self.state == PlayerState::Walking {
            self.state = PlayerState::Idle;
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
