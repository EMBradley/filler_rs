use super::game::Player;
use iced::{color, Color};
use std::ops::{Index, IndexMut};

pub const ROW_COUNT: usize = 7;
pub const COL_COUNT: usize = 8;
pub const LAST_ROW: usize = ROW_COUNT - 1;
pub const LAST_COL: usize = COL_COUNT - 1;
pub const COLORS: [TileColor; 6] = [
    TileColor::Red,
    TileColor::Yellow,
    TileColor::Green,
    TileColor::Blue,
    TileColor::Purple,
    TileColor::Black,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Red = 0,
    Yellow = 1,
    Green = 2,
    Blue = 3,
    Purple = 4,
    Black = 5,
}

impl Default for TileColor {
    fn default() -> Self {
        TileColor::Black
    }
}

impl From<TileColor> for Color {
    fn from(color: TileColor) -> Self {
        match color {
            TileColor::Black => color!(0x333333),
            TileColor::Red => color!(0xD0312D),
            TileColor::Yellow => color!(0xFFFD37),
            TileColor::Blue => color!(0x318CE7),
            TileColor::Green => color!(0x3CB043),
            TileColor::Purple => color!(0xA32CC4),
        }
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

    pub fn get_neighbors(&self) -> Vec<Self> {
        let &Self { row: i, col: j } = self;
        let mut neighbors = Vec::with_capacity(4);

        if i > 0 {
            neighbors.push(Coordinates::new(i - 1, j));
        }
        if j > 0 {
            neighbors.push(Coordinates::new(i, j - 1));
        }
        if i + 1 < ROW_COUNT {
            neighbors.push(Coordinates::new(i + 1, j));
        }
        if j + 1 < COL_COUNT {
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
