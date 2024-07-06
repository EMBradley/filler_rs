use super::game::Player;
use iced::{color, Color};

pub const TILE_SIZE: f32 = 75.0;

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
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    pub owner: Option<Player>,
    pub color: TileColor,
    pub coordinates: Coordinates,
}
