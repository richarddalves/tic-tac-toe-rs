// src/game/mod.rs

mod error;

use crate::board::Board;
use crate::player::Player;

pub use error::GameError;

pub enum GameState {
    Won,
    Draw,
    Neutral,
}

pub struct Game {
    pub round: u32,
    pub board: Board,
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            round: 1,
            board: Board::default(),
            player1,
            player2,
        }
    }

    pub fn current_player(&self) -> &Player {
        if self.round % 2 == 0 {
            &self.player2
        } else {
            &self.player1
        }
    }

    pub fn play(&mut self, position: usize) -> Result<GameState, GameError> {
        let current_symbol = self.current_player().symbol;

        if self.board.is_busy_cell(position) {
            return Err(GameError::BusyCell);
        }

        let row = (position - 1) / 3;
        let col = (position - 1) % 3;

        self.board[row][col] = Some(current_symbol);

        if self.check_victory() {
            Ok(GameState::Won)
        } else if self.board.is_full() {
            Ok(GameState::Draw)
        } else {
            // game continues
            self.round += 1;
            Ok(GameState::Neutral)
        }
    }

    fn check_victory(&self) -> bool {
        // first cheking the lines combination
        for row in self.board.cells() {
            let symbol_a = row[0];
            let symbol_b = row[1];
            let symbol_c = row[2];

            if symbol_a.is_some() && symbol_a == symbol_b && symbol_b == symbol_c {
                return true;
            }
        }

        // checking for each column combination
        for col in 0..=2 {
            let symbol_a = self.board[0][col];
            let symbol_b = self.board[1][col];
            let symbol_c = self.board[2][col];

            if symbol_a.is_some() && symbol_a == symbol_b && symbol_b == symbol_c {
                return true;
            }
        }

        // checking for diagonals
        let symbol_1 = self.board[0][0];
        let symbol_5 = self.board[1][1];
        let symbol_9 = self.board[2][2];

        if symbol_1.is_some() && symbol_1 == symbol_5 && symbol_5 == symbol_9 {
            return true;
        }

        let symbol_3 = self.board[0][2];
        let symbol_7 = self.board[2][0];
        if symbol_3.is_some() && symbol_3 == symbol_5 && symbol_5 == symbol_7 {
            return true;
        }

        false
    }
}
