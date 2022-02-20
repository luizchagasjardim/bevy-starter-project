use bevy::prelude::*;

use crate::state::AppState;

pub struct GameOver;

impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(game_over));
    }
}

fn game_over(
    mut commands: Commands,
    query: Query<Entity, Without<Camera>>,
    mut state: ResMut<State<AppState>>,
) {
    use crate::log::*;
    console_log!("game over");
    for id in query.iter() {
        commands.entity(id).despawn();
    }
    state.set(AppState::Game).unwrap();
}