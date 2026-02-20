// src/bot/mod.rs

//! Computer-controlled opponents
//! Each submodule implements a different difficulty level

pub mod easy;

/// The difficulty level of the computer opponent.
pub enum Difficulty {
    Easy, // Just random moves on available positions + TODO: tries to avoid player wins
    Medium, // TODO: 50% chance of a easy move + 50% chance of a hard move
    Hard, // TODO: Minimax (not yet implemented)
}
