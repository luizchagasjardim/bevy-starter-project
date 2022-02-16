#![feature(derive_default_enum)]

use bevy::prelude::*;

mod sprite;

use sprite::SpriteHandles;

mod state;
use state::*;

#[macro_use]
extern crate lazy_static;

mod log;
use log::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

    console_log!("Starting Game!");
    App::new()
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugin(Loading)
        .add_plugin(Game)
        .run();
}



