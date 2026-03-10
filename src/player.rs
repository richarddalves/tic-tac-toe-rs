// src/player.rs

use crate::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub symbol: Symbol,
}

impl Player {
    pub fn new(symbol: Symbol) -> Self {
        Self { symbol }
    }
}
