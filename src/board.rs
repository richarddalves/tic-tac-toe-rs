// src/board.rs

use crate::symbol::Symbol;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct Board {
    cells: [[Option<Symbol>; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn is_full(&self) -> bool {
        let mut count = 0;
        for row in self.cells {
            for cell in row {
                if let Some(_) = cell {
                    count += 1;
                }
            }
        }

        count == 9
    }

    pub fn is_busy_cell(&self, cell: usize) -> bool {
        let row = (cell - 1) / 3;
        let col = (cell - 1) % 3;

        match self[row][col] {
            Some(_) => true,
            None => false,
        }
    }

    // temp solution while i dont implement iterator for cells
    pub fn cells(&self) -> [[Option<Symbol>; 3]; 3] {
        self.cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells {
            for cell in row {
                match cell {
                    Some(symbol) => write!(f, "{symbol}  ")?,
                    None => write!(f, "_  ")?,
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Index<usize> for Board {
    type Output = [Option<Symbol>; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}
