use game::Game;
use iced::{Sandbox, Settings};

mod game;
mod grid;

fn main() -> iced::Result {
    Game::run(Settings::default())
}
