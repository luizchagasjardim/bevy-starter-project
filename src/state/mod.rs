#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    PreLoad,
    Loading,
    Game,
    GameOver,
}

mod game;
pub use game::Game;
mod loading;
pub use loading::Loading;
mod game_over;
pub use game_over::GameOver;
