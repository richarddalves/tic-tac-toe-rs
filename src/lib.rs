// src/lib.rs

//! Main library for the tic-tac-toe game.
//!
//! Exposes the public types used by the binary and by external consumers.

mod board;
pub mod bot;
mod error;
mod game;
mod player;

pub use board::{Board, Position};
pub use error::GameError;
pub use game::{Game, GameResult};
pub use player::{Player, Symbol};
