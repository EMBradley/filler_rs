use super::game::Player;
use iced::{color, Color};
use std::ops::{Index, IndexMut};

pub const ROWS: usize = 7;
pub const COLS: usize = 8;
pub const LAST_ROW: usize = ROWS - 1;
pub const LAST_COL: usize = COLS - 1;

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

impl From<u8> for TileColor {
    fn from(value: u8) -> Self {
        match value % 6 {
            0 => TileColor::Red,
            1 => TileColor::Yellow,
            2 => TileColor::Green,
            3 => TileColor::Blue,
            4 => TileColor::Purple,
            5 => TileColor::Black,
            _ => unreachable!(),
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
        let i = i as isize;
        let j = j as isize;
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        directions
            .iter()
            .filter_map(|(delta_i, delta_j)| {
                let i_new = i + delta_i;
                let j_new = j + delta_j;
                if (0..ROWS as isize).contains(&i_new) && (0..COLS as isize).contains(&j_new) {
                    Some(Coordinates {
                        row: i_new as usize,
                        col: j_new as usize,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    pub owner: Option<Player>,
    pub color: TileColor,
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, Copy)]
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
