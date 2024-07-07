use super::grid::{Coordinates, Grid, Tile, TileColor};
use iced::{
    widget::{button::Button, canvas::Canvas, column, row, text, Container},
    Alignment, Element, Length, Padding, Sandbox,
};
use rand::prelude::*;

const TILE_SIZE: f32 = 75.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    fn alternate(self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    current_player: Player,
    score: (usize, usize),
    grid: Grid,
}

impl Game {
    fn player_start_tile(&self, player: Player) -> Tile {
        match player {
            Player::One => self.grid[Grid::LAST_ROW][0],
            Player::Two => self.grid[0][Grid::LAST_COL],
        }
    }

    fn player_color(&self, player: Player) -> TileColor {
        self.player_start_tile(player).color
    }

    fn player_tile_coordinates(&self, player: Player) -> Vec<Coordinates> {
        self.grid
            .into_iter()
            .flat_map(|row| {
                row.into_iter().filter_map(|tile| {
                    if tile.owner? == player {
                        Some(tile.coordinates)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn disabled_colors(&self) -> [TileColor; 2] {
        [
            self.player_color(Player::One),
            self.player_color(Player::Two),
        ]
    }
}

impl Sandbox for Game {
    type Message = TileColor;

    fn new() -> Self {
        // Generate a grid of randomly colored tiles, assigning the bottom left tile to player one
        // and assigning the top right tile to player two
        let mut rng = thread_rng();
        let mut tiles = Grid::default();

        for row in 0..Grid::ROW_COUNT {
            for col in 0..Grid::COL_COUNT {
                let available_colors = TileColor::ALL.into_iter().filter(|&color| {
                    (row == 0 || color != tiles[row - 1][col].color)
                        && (col == 0 || color != tiles[row][col - 1].color)
                });
                let color = available_colors.choose(&mut rng).unwrap();
                let owner = match (row, col) {
                    (Grid::LAST_ROW, 0) => Some(Player::One),
                    (0, Grid::LAST_COL) => Some(Player::Two),
                    _ => None,
                };

                tiles[row][col] = Tile {
                    owner,
                    color,
                    coordinates: Coordinates::new(row, col),
                };
            }
        }

        // Ensure that player 1 and player 2 have different colors
        let available_colors = TileColor::ALL.into_iter().filter(|&color| {
            color != tiles[0][Grid::LAST_COL].color
                && color != tiles[Grid::LAST_ROW][1].color
                && color != tiles[Grid::LAST_ROW - 1][0].color
        });
        let color = available_colors.choose(&mut rng).unwrap();
        tiles[Grid::LAST_ROW][0].color = color;

        Self {
            current_player: Player::One,
            score: (1, 1),
            grid: tiles,
        }
    }

    fn title(&self) -> String {
        "Filler".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        debug_assert_ne!(message, self.player_color(Player::One));
        debug_assert_ne!(message, self.player_color(Player::Two));

        for coordinates in self.player_tile_coordinates(self.current_player) {
            self.grid[coordinates].color = message;

            for neighbor_coordinates in coordinates.neighbors() {
                let neighbor = &mut self.grid[neighbor_coordinates];
                if neighbor.color == message && neighbor.owner.is_none() {
                    neighbor.owner = Some(self.current_player);
                }
            }
        }

        self.score = match self.current_player {
            Player::One => (self.player_score(Player::One), self.score.1),
            Player::Two => (self.score.0, self.player_score(Player::Two)),
        };

        self.current_player = self.current_player.alternate();
    }

    fn view(&self) -> Element<Self::Message> {
        let score_board = text(format!("Score: {0} | {1}", self.score.0, self.score.1));
        let to_play = {
            let player = match self.current_player {
                Player::One => "Player 1",
                Player::Two => "Player 2",
            };
            text(format!("{player}'s turn"))
        };
        let info = row![score_board, to_play]
            .spacing(TILE_SIZE * 6.0)
            .height(Length::FillPortion(1));

        let grid = Container::new(
            Canvas::new(&self.grid)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(8))
        .center_x()
        .center_y();

        let buttons = row(TileColor::ALL.into_iter().map(|color| {
            let is_enabled = !self.disabled_colors().contains(&color);
            let message = if is_enabled { Some(color) } else { None };
            let size = if is_enabled {
                TILE_SIZE
            } else {
                TILE_SIZE * 0.75
            };
            let button = Button::new("")
                .width(size)
                .height(size)
                .padding(TILE_SIZE - size)
                .style(color)
                .on_press_maybe(message);
            button.into()
        }))
        .align_items(Alignment::Center)
        .spacing(TILE_SIZE * 0.25)
        .height(Length::FillPortion(2))
        .width(Length::Shrink);

        column![info, grid, buttons]
            .align_items(Alignment::Center)
            .padding(Padding::from(25.0))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Player, Sandbox};

    #[test]
    fn test_players_start_different_colors() {
        for _ in 0..100 {
            let game = Game::new();
            assert_ne!(
                game.player_color(Player::One),
                game.player_color(Player::Two)
            );
        }
    }

    #[test]
    fn test_adjacent_tiles_start_different_colors() {
        for _ in 0..100 {
            let game = Game::new();
            for row in game.grid {
                for cell in row {
                    let color = cell.color;
                    for neighbor_coordinates in cell.coordinates.neighbors() {
                        let neighbor_color = game.grid[neighbor_coordinates].color;
                        assert_ne!(color, neighbor_color);
                    }
                }
            }
        }
    }
}
