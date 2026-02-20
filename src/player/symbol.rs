// src/player/symbol.rs

//! Defines the symbol used by each player on the board: X or O

use std::fmt::Display;

/// The two possible symbols in tic-tac-toe
///
/// Derives `Copy` so it can be passed by value freely without needing clone or references.
/// Derives `PartialEq` so symbols can be compared for equality across cells to determine the current player.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    X,
    O,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
}
