use super::game::Player;
use iced::{color, Color};

pub const CELL_SIZE: f32 = 75.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellColor {
    Red = 0,
    Yellow = 1,
    Green = 2,
    Blue = 3,
    Purple = 4,
    Black = 5,
}

impl Default for CellColor {
    fn default() -> Self {
        CellColor::Black
    }
}

impl From<CellColor> for Color {
    fn from(color: CellColor) -> Self {
        match color {
            CellColor::Black => color!(0x333333),
            CellColor::Red => color!(0xD0312D),
            CellColor::Yellow => color!(0xFFFD37),
            CellColor::Blue => color!(0x004F98),
            CellColor::Green => color!(0x3CB043),
            CellColor::Purple => color!(0xA32CC4),
        }
    }
}

impl From<u8> for CellColor {
    fn from(value: u8) -> Self {
        match value % 6 {
            0 => CellColor::Red,
            1 => CellColor::Yellow,
            2 => CellColor::Green,
            3 => CellColor::Blue,
            4 => CellColor::Purple,
            5 => CellColor::Black,
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
pub struct Cell {
    pub owner: Option<Player>,
    pub color: CellColor,
    pub coordinates: Coordinates,
}
