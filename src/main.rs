use bevy::prelude::*;

mod sprite;
use sprite::SpriteHandles;

mod state;
use state::*;

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugin(Loading)
        .add_plugin(Game)
        .run();
}



