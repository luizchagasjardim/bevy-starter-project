use bevy::prelude::*;
use super::direction::Direction;
use super::hitbox::{PlayerGroundHitbox, PlayerEnemyHitbox};
use super::velocity::Velocity;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player_character: PlayerCharacter,
    pub controls: Controls,
    pub ground_hitbox: PlayerGroundHitbox,
    pub enemy_hitbox: PlayerEnemyHitbox,
    pub velocity: Velocity,
    pub direction: Direction,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum State {
    #[default]
    Idle,
    Walking,
    InTheAir(u8),
}

#[derive(Component, Default)]
pub struct PlayerCharacter {
    state: State,
    previous_state: State,
}

impl PlayerCharacter {
    const MAX_JUMP_COUNT: u8 = 2;
    fn get_jump_count(&self) -> u8 {
        if let State::InTheAir(jump_count) = self.state {
            jump_count
        } else {
            0
        }
    }
    pub fn update_walk_state(&mut self, x_velocity: f32) {
        if let State::InTheAir(_) = self.state {
            return;
        }
        self.state = if x_velocity != 0.0 {
                State::Walking
            } else {
                State::Idle
            };
    }
    pub fn try_jump(&mut self) -> Result<(), ()> {
        let jump_count = self.get_jump_count();
        if jump_count < Self::MAX_JUMP_COUNT {
            self.state = State::InTheAir(jump_count+1);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn hit_ground(&mut self) {
        if let State::InTheAir(_) = self.state {
            self.state = State::Idle;
        }
    }
    pub fn update_spritesheet(&mut self) -> Option<&str> {
        if self.state == self.previous_state {
            return None;
        }
        self.previous_state = self.state;
        let spritesheet = match self.state {
            State::Idle => "green idle",
            State::Walking => "green walk",
            State::InTheAir(_) => "green jump",
        };
        Some(spritesheet)
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
