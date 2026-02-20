// src/main.rs

//! Entry point of the program.
//!
//! Collects player information via the terminal and starts the match.

use std::io::{self, Write};
use ttt::{Game, GameError, GameResult, Player, Symbol};

fn main() {
    println!("========== Tic-Tac-Toe ==========");

    let player1 = ask_name("Enter the name of the first player: ");
    let symbol1 = ask_symbol("Choose your symbol [X or O]: ");
    let player2 = ask_name("Enter the name of the second player: ");

    let (player1, player2) = Player::create_pair(player1, player2, symbol1);

    let game = Game::new(player1, player2);

    run(game);
}

fn run(mut game: Game) {
    loop {
        println!("{}", &game.board);

        let position = ask_position(&format!(
            "{}, enter your next move (1-9): ",
            game.current_player()
        ));

        match game.play(position) {
            Ok(GameResult::Victory) => {
                println!("{}", &game.board);
                println!("========= {} won!!! =========", game.current_player());
                break;
            }

            Ok(GameResult::Draw) => {
                println!("{}", &game.board);
                println!("========= DRAW!!! =========");
                break;
            }

            Ok(GameResult::InProgress) => continue,

            Err(e) => eprintln!("{e}"),
        }
    }
}

/// Reads a line from the terminal and returns the trimmed content.
fn ask_name(msg: &str) -> String {
    let mut input_buffer = String::new();

    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        match io::stdin().read_line(&mut input_buffer) {
            Ok(_) => return input_buffer.trim().to_string(),
            Err(_) => continue,
        }
    }
}

/// Reads the symbol chosen by player 1 (X or O), repeating until a valid input is received.
fn ask_symbol(msg: &str) -> Symbol {
    let mut input_buffer = String::new();

    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();
        input_buffer = input_buffer.trim().to_lowercase();

        if input_buffer == "x" {
            return Symbol::X;
        } else if input_buffer == "o" {
            return Symbol::O;
        } else {
            continue; // invalid input, loop continues
        }
    }
}

/// Reads a board position (1-9) from the terminal, repeating until a valid input is received.
fn ask_position(msg: &str) -> usize {
    let mut input_buffer = String::new();

    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();

        match input_buffer.trim().parse::<usize>() {
            Ok(position) if position >= 1 && position <= 9 => return position,
            _ => {
                eprintln!("{}", GameError::InvalidPosition);
                continue;
            }
        }
    }
}
