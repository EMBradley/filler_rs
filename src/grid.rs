use super::game::Player;
use iced::{
    self, color, mouse, theme,
    widget::{
        button,
        canvas::{self, Frame, Path, Program},
    },
    Background, Border, Point, Shadow, Size, Vector,
};
use std::{
    array::IntoIter,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TileColor {
    #[default]
    Black,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl TileColor {
    pub const ALL: [Self; 6] = [
        TileColor::Red,
        TileColor::Yellow,
        TileColor::Green,
        TileColor::Blue,
        TileColor::Purple,
        TileColor::Black,
    ];
}

impl From<TileColor> for iced::Color {
    fn from(color: TileColor) -> Self {
        match color {
            TileColor::Black => color!(0x33_33_33),
            TileColor::Red => color!(0xD0_31_2D),
            TileColor::Yellow => color!(0xFF_FD_37),
            TileColor::Blue => color!(0x31_8C_E7),
            TileColor::Green => color!(0x3C_B0_43),
            TileColor::Purple => color!(0xA3_2C_C4),
        }
    }
}

impl button::StyleSheet for TileColor {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let color = iced::Color::from(*self);
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl Coordinates {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let &Self { row: i, col: j } = self;
        let mut neighbors = Vec::with_capacity(4);

        if i > 0 {
            neighbors.push(Coordinates::new(i - 1, j));
        }
        if j > 0 {
            neighbors.push(Coordinates::new(i, j - 1));
        }
        if i < Grid::LAST_ROW {
            neighbors.push(Coordinates::new(i + 1, j));
        }
        if j < Grid::LAST_COL {
            neighbors.push(Coordinates::new(i, j + 1));
        }

        neighbors
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    pub owner: Option<Player>,
    pub color: TileColor,
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Grid(pub [[Tile; 8]; 7]);

impl Grid {
    pub const ROW_COUNT: usize = 7;
    pub const COL_COUNT: usize = 8;
    pub const LAST_ROW: usize = 6;
    pub const LAST_COL: usize = 7;
}

impl Index<Coordinates> for Grid {
    type Output = Tile;

    fn index(&self, coordinates: Coordinates) -> &Self::Output {
        &self[coordinates.row][coordinates.col]
    }
}

impl IndexMut<Coordinates> for Grid {
    fn index_mut(&mut self, coordinates: Coordinates) -> &mut Self::Output {
        &mut self[coordinates.row][coordinates.col]
    }
}

impl Index<usize> for Grid {
    type Output = [Tile; 8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IntoIterator for Grid {
    type Item = [Tile; 8];
    type IntoIter = IntoIter<[Tile; 8], 7>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Message> Program<Message> for Grid {
    type State = ();

    #[allow(clippy::cast_precision_loss)]
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<<iced::Renderer as canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let max_tile_width = bounds.size().width / 8.0;
        let max_tile_height = bounds.size().height / 7.0;
        let tile_size = if max_tile_width < max_tile_height {
            max_tile_width
        } else {
            max_tile_height
        };

        let x_offset = (bounds.size().width - (8.0 * tile_size)) / 2.0;
        let y_offset = (bounds.size().height - (7.0 * tile_size)) / 2.0;

        for row in *self {
            for cell in row {
                let Coordinates { row: i, col: j } = cell.coordinates;
                let x = x_offset + j as f32 * tile_size;
                let y = y_offset + i as f32 * tile_size;
                let top_left = Point { x, y };
                let size = Size::new(tile_size, tile_size);
                let square = Path::rectangle(top_left, size);
                frame.fill(&square, iced::Color::from(cell.color));
            }
        }

        vec![frame.into_geometry()]
    }
}
