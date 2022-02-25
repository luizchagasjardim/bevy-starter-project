use bevy::prelude::*;

use crate::AppState;
use crate::controls::Controls;

pub struct Pause;

impl Plugin for Pause {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(pause))
            .add_system_set(SystemSet::on_update(AppState::Pause).with_system(pause));
    }
}

fn pause(
    mut state: ResMut<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    query: Query<&Controls>,
) {
    for controls in query.iter() {
        if input.just_pressed(controls.pause) {
            match state.current() {
                AppState::Game => state.push(AppState::Pause).unwrap(),
                AppState::Pause => state.pop().unwrap(),
                _ => unimplemented!(),
            }
            input.reset(controls.pause);
        }
    }
}
