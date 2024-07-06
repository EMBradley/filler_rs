use super::tile::{Coordinates, Grid, Tile, TileColor, TILE_SIZE};
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

#[derive(Debug, Clone, Copy)]
pub enum Player {
    One,
    Two,
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
    pub grid: Grid,
}

impl Game {
    fn get_player_color(&self, player: Player) -> TileColor {
        match player {
            Player::One => self.grid[6][0].color,
            Player::Two => self.grid[0][7].color,
        }
    }
    fn disabled_colors(&self) -> [TileColor; 2] {
        [
            self.get_player_color(Player::One),
            self.get_player_color(Player::Two),
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
            let row: [Tile; 8] = std::array::from_fn(|j| {
                let color = TileColor::from(rng.gen_range(0..6));
                let owner = match (i, j) {
                    (6, 0) => Some(Player::One),
                    (0, 7) => Some(Player::Two),
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
        if tiles[6][0].color == tiles[0][7].color {
            let available_colors = (0..6)
                .map(TileColor::from)
                .filter(|&color| color != tiles[6][0].color)
                .collect::<Vec<_>>();
            let i = rng.gen_range(0..available_colors.len());
            tiles[0][7].color = available_colors[i];
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
        assert_ne!(message, self.get_player_color(Player::One));
        assert_ne!(message, self.get_player_color(Player::Two));
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
                game.get_player_color(Player::One),
                game.get_player_color(Player::Two)
            );
        }
    }
}
