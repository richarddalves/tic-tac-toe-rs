// src/main.rs

use std::io::{self, Write};
use std::num::ParseIntError;
use std::process;

use tic_tac_toe_rs::game::{Game, GameState};
use tic_tac_toe_rs::player::Player;
use tic_tac_toe_rs::symbol::Symbol;

// It was created here because it works only with stdin
#[derive(Debug)]
enum InputError {
    InvalidSymbol,
    InvalidPosition,
    #[allow(dead_code)]
    IOError(String),
}

impl From<io::Error> for InputError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl From<ParseIntError> for InputError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidPosition
    }
}

fn main() {
    println!("========== TIC TAC TOE ==========");

    let (player1, player2) = match ask_for_symbol("Insert player 1 symbol [X or O]: ") {
        Ok(symbol) => (Player::new(symbol), Player::new(symbol.opposite())),

        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    };

    if let Err(err) = run(Game::new(player1, player2)) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn run(mut game: Game) -> Result<(), InputError> {
    game.show_board();
    
    loop {
        let current_player = game.current_player();

        let new_position = if &game.player1 == current_player {
            ask_for_position("Player 1, insert your next move [1-9]: ")
        } else {
            ask_for_position("Player 2, insert your next move [1-9]: ")
        }?;

        if let Ok(game_result) = game.play(new_position) {
            game.show_board();

            match game_result {
                GameState::Neutral => continue,
                GameState::Won => {
                    println!("Congratulations! You won!!");
                    break;
                }
                GameState::Draw => {
                    println!("Draw!");
                    break;
                }
            }
        } else {
            println!("This position is already occupied. Try again!");
            continue;
        }
    }

    Ok(())
}

fn ask_for_symbol(msg: &str) -> Result<Symbol, InputError> {
    let mut buf = String::new();

    print!("{msg}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buf)?;

    match buf.to_lowercase().trim() {
        "x" => Ok(Symbol::X),
        "o" => Ok(Symbol::O),
        _ => Err(InputError::InvalidSymbol),
    }
}

fn ask_for_position(msg: &str) -> Result<usize, InputError> {
    let mut buf = String::new();

    print!("{msg}");
    io::stdout().flush()?;

    io::stdin().read_line(&mut buf)?;

    match buf.trim().parse::<usize>()? {
        n if n >= 1 && n <= 9 => Ok(n),
        _ => Err(InputError::InvalidPosition),
    }
}
