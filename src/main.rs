use game::Game;
use iced::{Sandbox, Settings};

mod cell;
mod game;

fn main() {
    let _ = Game::run(Settings::default());
}
