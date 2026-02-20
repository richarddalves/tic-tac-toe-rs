// src/game.rs

//! Defines the game engine: turn management, move application, and result detection

use crate::{Board, Player, Symbol, error::GameError};

/// The possible result after a move
#[derive(Debug, PartialEq, Eq)]
pub enum GameResult {
    /// A player completed a winning combination
    Victory,

    /// All 9 cells were filled with no winner
    Draw,

    /// The game is still in progress
    InProgress,
}

/// The 8 possible winning combinations: 3 rows, 3 columns, 2 diagonals.
///
/// Each sub-array uses the user-facing positions (1-9).
const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    [1, 2, 3], // row 1
    [4, 5, 6], // row 2
    [7, 8, 9], // row 3
    [1, 4, 7], // column 1
    [2, 5, 8], // column 2
    [3, 6, 9], // column 3
    [1, 5, 9], // main diagonal
    [3, 5, 7], // secondary diagonal
];

/// Represents an ongoing game, tracking the board, both players, and whose turn it is.
#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    player1: Player,
    player2: Player,
    current_turn: Symbol,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        let board = Board::new();
        let current_turn = player1.symbol;

        Self {
            board,
            player1,
            player2,
            current_turn,
        }
    }

    /// Returns a reference to the player whose turn it currently is.
    pub fn current_player(&self) -> &Player {
        if self.current_turn == self.player1.symbol {
            &self.player1
        } else {
            &self.player2
        }
    }

    /// Checks whether any of the 8 winning combinations is completed on the board.
    fn check_victory(&self) -> bool {
        for combination in WINNING_COMBINATIONS {
            let symbol_a = &self.board[combination[0]].symbol;
            let symbol_b = &self.board[combination[1]].symbol;
            let symbol_c = &self.board[combination[2]].symbol;

            // All 3 must be Some and hold the same value
            if symbol_a.is_some() && symbol_a == symbol_b && symbol_b == symbol_c {
                return true;
            }
        }

        false
    }

    /// Applies a move at the given position for the current player.
    ///
    /// Returns `Err` if the position is invalid or already occupied.
    /// Returns `Ok(GameResult)` indicating the state of the game after the move.
    pub fn play(&mut self, position: usize) -> Result<GameResult, GameError> {
        self.board.mark(position, self.current_turn)?;

        if self.check_victory() {
            Ok(GameResult::Victory)
        } else if self.board.is_full() {
            Ok(GameResult::Draw)
        } else {
            // Only switch turns if the game is still ongoing
            self.switch_turn();
            Ok(GameResult::InProgress)
        }
    }

    fn switch_turn(&mut self) {
        self.current_turn = if self.current_turn == Symbol::X {
            Symbol::O
        } else {
            Symbol::X
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        let (p1, p2) = Player::create_pair("Alice".to_string(), "Bob".to_string(), Symbol::X);
        Game::new(p1, p2)
    }

    #[test]
    fn player1_goes_first() {
        let game = test_game();
        assert_eq!(game.current_player().name, "Alice");
    }

    #[test]
    fn turn_switches_after_move() {
        let mut game = test_game();
        game.play(1).unwrap();
        assert_eq!(game.current_player().name, "Bob");
    }

    #[test]
    fn row_victory() {
        let mut game = test_game();
        game.play(1).unwrap(); // X
        game.play(4).unwrap(); // O
        game.play(2).unwrap(); // X
        game.play(5).unwrap(); // O
        let result = game.play(3).unwrap(); // X completes row 1-2-3
        assert_eq!(result, GameResult::Victory);
    }

    #[test]
    fn column_victory() {
        let mut game = test_game();
        game.play(1).unwrap(); // X
        game.play(2).unwrap(); // O
        game.play(4).unwrap(); // X
        game.play(5).unwrap(); // O
        let result = game.play(7).unwrap(); // X completes column 1-4-7
        assert_eq!(result, GameResult::Victory);
    }

    #[test]
    fn main_diagonal_victory() {
        let mut game = test_game();
        game.play(1).unwrap(); // X
        game.play(2).unwrap(); // O
        game.play(5).unwrap(); // X
        game.play(3).unwrap(); // O
        let result = game.play(9).unwrap(); // X completes diagonal 1-5-9
        assert_eq!(result, GameResult::Victory);
    }

    #[test]
    fn secondary_diagonal_victory() {
        let mut game = test_game();
        game.play(3).unwrap(); // X
        game.play(1).unwrap(); // O
        game.play(5).unwrap(); // X
        game.play(2).unwrap(); // O
        let result = game.play(7).unwrap(); // X completes diagonal 3-5-7
        assert_eq!(result, GameResult::Victory);
    }

    #[test]
    fn draw_when_board_is_full() {
        let mut game = test_game();
        // Resulting board with no winner:
        // X O X
        // X X O
        // O X O
        game.play(1).unwrap(); // X
        game.play(2).unwrap(); // O
        game.play(3).unwrap(); // X
        game.play(6).unwrap(); // O
        game.play(4).unwrap(); // X
        game.play(7).unwrap(); // O
        game.play(5).unwrap(); // X
        game.play(9).unwrap(); // O
        let result = game.play(8).unwrap(); // X — ninth move, board full
        assert_eq!(result, GameResult::Draw);
    }
}
