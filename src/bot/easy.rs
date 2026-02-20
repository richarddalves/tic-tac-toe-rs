// src/bot/easy.rs

use rand::seq::{IndexedRandom};

use crate::{Board};

/// Choses a random available position on the board.
/// Panics if the board is full (this should never be called in that case).
pub fn random_move(board: &Board) -> usize {
    let available = board.available_positions();

    *available
        .choose(&mut rand::rng())
        .expect("No available positions, board is full")
}
