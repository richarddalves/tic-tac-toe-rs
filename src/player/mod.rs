// src/player/mod.rs

//! Defines the player and the logic for initializing a match

mod symbol;
use std::fmt::Display;

pub use symbol::Symbol;

/// A participant in the game, identified by name and the symbol they use.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Player {
    pub name: String,
    pub symbol: Symbol,
}

impl Player {
    fn new(name: String, symbol: Symbol) -> Self {
        Self { name, symbol }
    }

    /// Initializes the two players for a match.
    ///
    /// Player 1 chooses their symbol.
    /// Player 2's symbol is automatically determined as the opposite.
    pub fn create_pair(name1: String, name2: String, symbol1: Symbol) -> (Self, Self) {
        let symbol2 = if symbol1 == Symbol::X {
            Symbol::O
        } else {
            Symbol::X
        };

        let player1 = Self::new(name1, symbol1);
        let player2 = Self::new(name2, symbol2);

        (player1, player2)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
