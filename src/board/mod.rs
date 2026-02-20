// src/board/mod.rs

//! Defines the board and its core operations: marking cells and checking capacity

mod cell;
mod position;

pub use cell::Cell;
pub use position::Position;

use std::fmt::Display;
use std::ops;

use crate::{Symbol, error::GameError};

/// A 3x3 board represented as a flat sequence of 9 cells.
///
/// Internally uses indices 0 to 8, but exposes an interface with positions 1 to 9
/// (via `Index<usize>`) to match what the user types.
#[derive(Debug, Clone)]
pub struct Board {
    cells: [Cell; 9],
    occupied_cells: u8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [
                Cell::new(Position::new(1)),
                Cell::new(Position::new(2)),
                Cell::new(Position::new(3)),
                Cell::new(Position::new(4)),
                Cell::new(Position::new(5)),
                Cell::new(Position::new(6)),
                Cell::new(Position::new(7)),
                Cell::new(Position::new(8)),
                Cell::new(Position::new(9)),
            ],
            occupied_cells: 0,
        }
    }

    /// Attempts to mark a cell. Returns `Err` if the position is invalid or already occupied.
    pub fn mark(&mut self, position: usize, symbol: Symbol) -> Result<(), GameError> {
        if position < 1 || position > 9 {
            return Err(GameError::InvalidPosition);
        }

        if let Some(_) = self[position].symbol {
            return Err(GameError::OccupiedPosition);
        }

        self[position].symbol = Some(symbol);
        self.occupied_cells += 1;

        Ok(())
    }

    /// Returns true if all 9 cells have been marked.
    pub fn is_full(&self) -> bool {
        self.occupied_cells == 9
    }

    /// Returns a list of all positions (1-9) that have not yet been marked
    pub fn available_positions(&self) -> Vec<usize> {
        let mut positions = Vec::new();

        for cell in &self.cells {
            if cell.symbol.is_none() {
                positions.push(cell.position.value);
            }
        }

        positions
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        // User-facing positions are 1-9; internal array uses 0-8
        self.cells.get(index - 1).expect("Position out of range")
    }
}

impl ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.cells
            .get_mut(index - 1)
            .expect("Position out of range")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            // Uses the cell's column to know when to break the line,
            // avoiding the need for an external counter
            match cell.position.column {
                0 | 1 => write!(f, "{}  ", cell)?,
                2 => writeln!(f, "{}", cell)?,
                _ => panic!("More columns than expected"),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Symbol, error::GameError};

    #[test]
    fn mark_empty_cell_succeeds() {
        let mut board = Board::new();
        assert!(board.mark(5, Symbol::X).is_ok());
        assert_eq!(board[5].symbol, Some(Symbol::X));
    }

    #[test]
    fn mark_occupied_cell_returns_error() {
        let mut board = Board::new();
        board.mark(5, Symbol::X).unwrap();
        assert_eq!(board.mark(5, Symbol::O), Err(GameError::OccupiedPosition));
    }

    #[test]
    fn position_zero_is_invalid() {
        let mut board = Board::new();
        assert_eq!(board.mark(0, Symbol::X), Err(GameError::InvalidPosition));
    }

    #[test]
    fn position_ten_is_invalid() {
        let mut board = Board::new();
        assert_eq!(board.mark(10, Symbol::X), Err(GameError::InvalidPosition));
    }

    #[test]
    fn is_full_after_nine_marks() {
        let mut board = Board::new();
        for i in 1..=9 {
            board.mark(i, Symbol::X).unwrap();
        }
        assert!(board.is_full());
    }

    #[test]
    fn is_not_full_with_eight_marks() {
        let mut board = Board::new();
        for i in 1..=8 {
            board.mark(i, Symbol::X).unwrap();
        }
        assert!(!board.is_full());
    }

    #[test]
    fn available_positions_decreases_after_mark() {
        let mut board = Board::new();
        assert_eq!(board.available_positions().len(), 9); // starts with 9 empty cells
        board.mark(5, Symbol::X).unwrap();
        assert_eq!(board.available_positions().len(), 8); // after mark, 8 empty cells
        assert!(!board.available_positions().contains(&5)); // position 5 is not available anymore
    }
}
