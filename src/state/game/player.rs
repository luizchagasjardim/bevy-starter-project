use bevy::prelude::*;

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