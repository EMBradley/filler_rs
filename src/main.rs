use game::Game;
use iced::{Sandbox, Settings};

mod game;
mod tile;

fn main() -> iced::Result {
    Game::run(Settings::default())
}
