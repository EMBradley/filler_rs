use std::collections::VecDeque;

use super::tile::{Coordinates, Grid, Tile, TileColor, COLS, LAST_COL, LAST_ROW, ROWS};
use iced::{
    mouse, theme,
    widget::{
        button::{self, Button},
        canvas::{self, Canvas, Frame, Path, Program},
        column, row, Container,
    },
    Alignment, Background, Border, Color, Element, Length, Point, Sandbox, Shadow, Size, Vector,
};
use rand::prelude::*;

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
        (0..ROWS)
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
        let mut tiles = Grid(std::array::from_fn(|i| {
            let row: [Tile; COLS] = std::array::from_fn(|j| {
                let color = TileColor::from(rng.gen_range(0..6));
                let owner = match (i, j) {
                    (LAST_ROW, 0) => Some(Player::One),
                    (0, LAST_COL) => Some(Player::Two),
                    _ => None,
                };
                Tile {
                    color,
                    owner,
                    coordinates: Coordinates::new(i, j),
                }
            });
            row
        }));

        // Ensure that player 1 and player 2 have different colors
        if tiles[LAST_ROW][0].color == tiles[0][LAST_COL].color {
            let available_colors = (0..6)
                .map(TileColor::from)
                .filter(|&color| color != tiles[LAST_ROW][0].color)
                .collect::<Vec<_>>();
            let i = rng.gen_range(0..available_colors.len());
            tiles[0][LAST_COL].color = available_colors[i];
        }

        Self {
            to_play: Player::One,
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

        self.to_play = self.to_play.alternate();
    }

    fn view(&self) -> Element<Self::Message> {
        let grid = Container::new(
            Canvas::new(&self.grid)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(4))
        .center_x()
        .center_y();
        let buttons = row((0..6).map(|n| {
            let color = TileColor::from(n);
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
        .spacing(10.0);
        column![grid, buttons].into()
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
}
