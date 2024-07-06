use game::Game;
use iced::{Sandbox, Settings};

mod game;
mod tile;

fn main() {
    let _ = Game::run(Settings::default());
}
