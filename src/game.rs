use super::tile::{
    Coordinates, Grid, Tile, TileColor, COLORS, COL_COUNT, LAST_COL, LAST_ROW, ROW_COUNT,
};
use iced::{
    mouse, theme,
    widget::{
        button::{self, Button},
        canvas::{self, Canvas, Frame, Path, Program},
        column, row, text, Container,
    },
    Alignment, Background, Border, Color, Element, Length, Point, Sandbox, Shadow, Size, Vector,
};
use rand::prelude::*;
use std::collections::VecDeque;

pub const TILE_SIZE: f32 = 75.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    fn alternate(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::One
    }
}

impl button::StyleSheet for TileColor {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let color = Color::from(*self);
        let shadow_offset = Vector::new(0.0, 0.0);
        let background = Some(Background::Color(color));
        let text_color = color;
        let border = Border::default();
        let shadow = Shadow::default();

        button::Appearance {
            shadow_offset,
            background,
            text_color,
            border,
            shadow,
        }
    }
}

impl From<TileColor> for theme::Button {
    fn from(color: TileColor) -> Self {
        Self::Custom(Box::new(color))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    to_play: Player,
    score: (usize, usize),
    grid: Grid,
}

impl Game {
    fn player_start_tile(&self, player: Player) -> Tile {
        match player {
            Player::One => self.grid[LAST_ROW][0],
            Player::Two => self.grid[0][LAST_COL],
        }
    }

    fn player_color(&self, player: Player) -> TileColor {
        self.player_start_tile(player).color
    }

    fn player_tile_coordinates(&self, player: Player) -> Vec<Coordinates> {
        (0..ROW_COUNT)
            .flat_map(|i| {
                let row = self.grid[i];
                row.iter()
                    .filter_map(|tile| match tile.owner {
                        Some(owner) if owner == player => Some(tile.coordinates),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
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

        for row in 0..ROW_COUNT {
            for col in 0..COL_COUNT {
                let coordinates = Coordinates::new(row, col);
                let available_colors = COLORS.into_iter().filter(|&color| {
                    (row == 0 || color != tiles[row - 1][col].color)
                        && (col == 0 || color != tiles[row][col - 1].color)
                });
                let color = available_colors.choose(&mut rng).unwrap();
                let owner = match (row, col) {
                    (LAST_ROW, 0) => Some(Player::One),
                    (0, LAST_COL) => Some(Player::Two),
                    _ => None,
                };
                let tile = Tile {
                    color,
                    owner,
                    coordinates,
                };
                tiles[coordinates] = tile;
            }
        }

        // Ensure that player 1 and player 2 have different colors
        let available_colors = COLORS.into_iter().filter(|&color| {
            color != tiles[0][LAST_COL].color
                && color != tiles[LAST_ROW][1].color
                && color != tiles[LAST_ROW - 1][0].color
        });
        let color = available_colors.choose(&mut rng).unwrap();
        tiles[LAST_ROW][0].color = color;

        Self {
            to_play: Player::One,
            score: (1, 1),
            grid: tiles,
        }
    }

    fn title(&self) -> String {
        "Filler".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        assert_ne!(message, self.player_color(Player::One));
        assert_ne!(message, self.player_color(Player::Two));

        let mut update_queue = VecDeque::from(self.player_tile_coordinates(self.to_play));

        while let Some(coordinates) = update_queue.pop_front() {
            if self.grid[coordinates].owner == Some(self.to_play) {
                self.grid[coordinates].color = message;
            } else {
                self.grid[coordinates].owner = Some(self.to_play);
            }

            let neighbors = coordinates.get_neighbors();
            let neighbors_to_update = neighbors.iter().copied().filter(|&neighbor_coordinates| {
                self.grid[neighbor_coordinates].color == message
                    && self.grid[neighbor_coordinates].owner.is_none()
            });
            update_queue.extend(neighbors_to_update);
        }

        let new_score = self.player_tile_coordinates(self.to_play).iter().count();
        self.score = match self.to_play {
            Player::One => (new_score, self.score.1),
            Player::Two => (self.score.0, new_score),
        };

        self.to_play = self.to_play.alternate();
    }

    fn view(&self) -> Element<Self::Message> {
        let score_board = text(format!("{0} | {1}", self.score.0, self.score.1));
        let to_play = {
            let player = match self.to_play {
                Player::One => "Player 1",
                Player::Two => "Player 2",
            };
            text(format!("{player}'s turn"))
        };
        let info = row![score_board, to_play].spacing(TILE_SIZE * 6.0);

        let grid = Container::new(
            Canvas::new(&self.grid)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(4))
        .center_x()
        .center_y();

        let buttons = row(COLORS.into_iter().map(|color| {
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
        .spacing(TILE_SIZE * 0.25);

        column![info, grid, buttons].into()
    }
}

impl<Message> Program<Message> for Grid {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<<iced::Renderer as canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let Grid(grid) = self;

        for row in grid {
            for cell in row {
                let Coordinates { row: i, col: j } = cell.coordinates;
                let x = j as f32 * TILE_SIZE;
                let y = i as f32 * TILE_SIZE;
                let top_left = Point { x, y };
                let size = Size {
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                };
                let square = Path::rectangle(top_left, size);
                frame.fill(&square, Color::from(cell.color));
            }
        }

        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Player, Sandbox, ROW_COUNT};

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
            for row in 0..ROW_COUNT {
                for cell in game.grid[row] {
                    let color = cell.color;
                    for neighbor_coordinates in cell.coordinates.get_neighbors() {
                        let neighbor_color = game.grid[neighbor_coordinates].color;
                        assert_ne!(color, neighbor_color);
                    }
                }
            }
        }
    }
}
