// src/error.rs

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    OccupiedPosition,
    InvalidPosition, // number out of 1-9 range
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidPosition => write!(
                f,
                "Invalid position. Please enter a number between 1 and 9."
            ),
            GameError::OccupiedPosition => {
                write!(
                    f,
                    "This position is already occupied. Try another position."
                )
            }
        }
    }
}
