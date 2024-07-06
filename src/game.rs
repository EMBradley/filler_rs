use std::ops::Index;

use super::cell::{Coordinates, Tile, TileColor, TILE_SIZE};
use iced::{
    mouse,
    widget::{
        canvas,
        canvas::{Frame, Path, Program},
        column, Container,
    },
    Color, Element, Length, Padding, Point, Sandbox, Size,
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

#[derive(Debug, Clone, Copy)]
pub struct Grid([[Tile; 8]; 7]);

impl Default for Grid {
    fn default() -> Self {
        Grid([[Tile::default(); 8]; 7])
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
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
            Player::One => self.grid[(6, 0)].color,
            Player::Two => self.grid[(0, 7)].color,
        }
    }
}

impl Sandbox for Game {
    type Message = TileColor;

    fn new() -> Self {
        let mut rng = thread_rng();
        let cells = Grid(std::array::from_fn(|i| {
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

        Self {
            to_play: Player::One,
            grid: cells,
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
        let grid = Container::new(canvas(&self.grid).width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::FillPortion(4))
            .center_x()
            .center_y();
        let controls = Container::new("TODO: Controls");
        column![grid, controls].into()
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
