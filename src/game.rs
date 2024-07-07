use super::grid::{Coordinates, Grid, Tile, TileColor};
use iced::{
    advanced::graphics::core::font,
    alignment::Horizontal,
    widget::{
        button::Button, canvas::Canvas, column, responsive, row, text, Container, Responsive, Row,
        Space,
    },
    Alignment, Element, Length, Padding, Sandbox,
};
use rand::prelude::*;

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

#[derive(Debug, Default, Clone, Copy)]
enum GameStatus {
    #[default]
    Incomplete,
    Won(Player),
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMessage {
    PlayAgain,
    Move(TileColor),
}

impl ToString for GameMessage {
    fn to_string(&self) -> String {
        match *self {
            Self::PlayAgain => format!("Play Again"),
            Self::Move(color) => format!("{color:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    current_player: Player,
    score: (u8, u8),
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
        (0..Grid::ROW_COUNT)
            .flat_map(|row| {
                (0..Grid::COL_COUNT).filter_map(move |col| {
                    let coordinates = Coordinates::new(row, col);
                    let tile = self.grid[coordinates];
                    if tile.owner? == player {
                        Some(coordinates)
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

    fn status(&self) -> GameStatus {
        if usize::from(self.score.0 + self.score.1) < Grid::ROW_COUNT * Grid::COL_COUNT {
            GameStatus::Incomplete
        } else if self.score.0 > self.score.1 {
            GameStatus::Won(Player::One)
        } else if self.score.0 < self.score.1 {
            GameStatus::Won(Player::Two)
        } else {
            GameStatus::Draw
        }
    }

    fn create_info_bar(&self) -> Row<GameMessage> {
        let game_status = self.status();

        let font = {
            let mut font = font::Font::default();
            font.weight = font::Weight::Bold;
            font
        };

        let middle = if let GameStatus::Incomplete = game_status {
            Element::from(Space::with_width(Length::Fill))
        } else {
            let reset_button = Button::new("Play Again").on_press(GameMessage::PlayAgain);
            Element::from(
                Container::new(reset_button)
                    .width(Length::Fill)
                    .center_x()
                    .center_y(),
            )
        };

        let score_board = text(format!("Score: {0} | {1}", self.score.0, self.score.1))
            .size(25.0)
            .font(font);

        let status_text = {
            let player = match self.current_player {
                Player::One => "Player 1",
                Player::Two => "Player 2",
            };

            let status_text = match game_status {
                GameStatus::Incomplete => format!("{player}'s turn"),
                GameStatus::Draw => "Draw!".to_string(),
                GameStatus::Won(Player::One) => "Player 1 Wins!".to_string(),
                GameStatus::Won(Player::Two) => "Player 2 Wins!".to_string(),
            };

            text(status_text)
                .font(font)
                .size(25.0)
                .height(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
        };

        row![score_board, middle, status_text]
            .padding(Padding::from([0.0, 50.0]))
            .height(Length::FillPortion(1))
            .width(Length::Fill)
    }

    fn create_buttons(&self) -> Responsive<GameMessage> {
        responsive(|size| {
            let game_in_progress = matches![self.status(), GameStatus::Incomplete];
            let max_tile_width = size.width / 6.0;
            let max_tile_height = size.height;
            let tile_size = max_tile_width.min(max_tile_height);

            let total_button_width = if game_in_progress {
                tile_size * 7.0
            } else {
                tile_size * 6.0
            };
            let spacer_width = (size.width - total_button_width) / 2.0;
            let left_spacer = Space::with_width(spacer_width);
            let right_spacer = Space::with_width(spacer_width);

            let padding = if game_in_progress {
                0.0
            } else {
                tile_size * 0.125
            };

            let button_row = row![left_spacer]
                .spacing(tile_size * 0.25)
                .padding(padding)
                .align_items(Alignment::Center);

            let buttons = TileColor::ALL.into_iter().map(|color| {
                let is_enabled = game_in_progress && !self.disabled_colors().contains(&color);
                let message = if is_enabled {
                    Some(GameMessage::Move(color))
                } else {
                    None
                };
                let size = if is_enabled {
                    tile_size
                } else {
                    tile_size * 0.75
                };
                let button = Button::new("")
                    .width(size)
                    .height(size)
                    .padding(tile_size - size)
                    .style(color)
                    .on_press_maybe(message);
                button.into()
            });

            button_row.extend(buttons).push(right_spacer).into()
        })
    }
}

impl Sandbox for Game {
    type Message = GameMessage;

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

                tiles[row][col] = Tile { owner, color };
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
        let GameMessage::Move(color) = message else {
            *self = Self::new();
            return;
        };

        assert_ne!(color, self.player_color(Player::One));
        assert_ne!(color, self.player_color(Player::Two));

        let mut new_score = match self.current_player {
            Player::One => self.score.0,
            Player::Two => self.score.1,
        };

        for coordinates in self.player_tile_coordinates(self.current_player) {
            self.grid[coordinates].color = color;

            for neighbor_coordinates in coordinates.neighbors() {
                let neighbor = &mut self.grid[neighbor_coordinates];
                if neighbor.color == color && neighbor.owner.is_none() {
                    neighbor.owner = Some(self.current_player);
                    new_score += 1;
                }
            }
        }

        match self.current_player {
            Player::One => self.score.0 = new_score,
            Player::Two => self.score.1 = new_score,
        };

        self.current_player = self.current_player.alternate();
    }

    fn view(&self) -> Element<Self::Message> {
        let info_bar = self.create_info_bar();

        let grid = Canvas::new(&self.grid)
            .width(Length::Fill)
            .height(Length::FillPortion(10));

        let buttons = self.create_buttons();

        column![info_bar, grid, buttons]
            .align_items(Alignment::Center)
            .padding(Padding::from(25.0))
            .spacing(25.0)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinates, Game, GameMessage, GameStatus, Grid, Player, Sandbox, TileColor};
    use rand::prelude::*;

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
            for i in 0..Grid::ROW_COUNT {
                for j in 0..Grid::COL_COUNT {
                    let coordinates = Coordinates::new(i, j);
                    let color = game.grid[coordinates].color;
                    for neighbor_coordinates in coordinates.neighbors() {
                        let neighbor_color = game.grid[neighbor_coordinates].color;
                        assert_ne!(color, neighbor_color);
                    }
                }
            }
        }
    }

    #[test]
    fn test_score_update() {
        let expensive_get_score = |game: Game, player: Player| {
            u8::try_from(game.player_tile_coordinates(player).len()).unwrap()
        };

        let mut rng = thread_rng();

        for _ in 0..25 {
            let mut game = Game::new();
            while let GameStatus::Incomplete = game.status() {
                let disabled_colors = game.disabled_colors();
                let available_colors = TileColor::ALL
                    .iter()
                    .filter(|color| !disabled_colors.contains(color));
                game.update(GameMessage::Move(
                    *available_colors.choose(&mut rng).unwrap(),
                ));

                let expected_score = (
                    expensive_get_score(game, Player::One),
                    expensive_get_score(game, Player::Two),
                );
                assert_eq!(expected_score, game.score);
            }
        }
    }
}
