use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world)
        .run();
}
