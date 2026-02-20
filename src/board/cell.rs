// src/board/cell.rs

use super::Position;
use super::Symbol;

use std::fmt::Display;

/// A cell on the board.
/// Stores its position and the symbol occupying it, if any.
#[derive(Debug, Clone)]
pub struct Cell {
    pub position: Position,
    pub symbol: Option<Symbol>,
}

impl Cell {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            symbol: None, // Cell starts empty
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.symbol {
            Some(symbol) => write!(f, "{}", symbol),
            None => write!(f, "_"), // Empty cell displays underscore
        }
    }
}
