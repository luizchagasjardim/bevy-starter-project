#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    PreLoad,
    Loading,
    Game,
}

mod game;
pub use game::Game;
mod loading;
pub use loading::Loading;
