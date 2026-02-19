// lib.rs
mod jogador;
mod tabuleiro;

use std::io::{self, Write};

pub use jogador::{Jogador, Simbolo};
pub use tabuleiro::{Posicao, ResultadoJogo, Tabuleiro};

pub struct Jogada {
    pub simbolo: Simbolo,
    pub posicao: Posicao,
}

impl Jogada {
    fn new(simbolo: Simbolo, posicao: usize) -> Self {
        Self {
            simbolo,
            posicao: Posicao::new(posicao),
        }
    }
}

pub fn run(jogador1: Jogador, jogador2: Jogador, mut mesa: Tabuleiro) {
    let mut input_buffer = String::new();
    println!("=== JOGO DA VELHA ===");
    println!("{mesa}");

    loop {
        let proximo_a_jogar = match mesa.ultimo_a_jogar {
            Some(simbolo) => {
                if simbolo == jogador1.simbolo {
                    &jogador2
                } else {
                    &jogador1
                }
            }

            None => &jogador1,
        };

        input_buffer.clear();
        print!("{proximo_a_jogar} insira sua próxima jogada: [Posição]: ",);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let posicao = match input_buffer.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= 9 => n,
            _ => {
                println!("Digite um número entre 1 e 9.");
                continue;
            }
        };

        match mesa.jogar(Jogada::new(proximo_a_jogar.simbolo, posicao)) {
            Ok(ResultadoJogo::EmAndamento) => println!("{mesa}"),

            Ok(ResultadoJogo::Vitoria(_)) => {
                println!("{mesa}");
                println!("========= {proximo_a_jogar} ganhou!!! =========");
                break;
            }

            Ok(ResultadoJogo::Empate) => {
                println!("{mesa}");
                println!("========= EMPATE =========");
                break;
            }

            Err(msg_erro) => {
                println!("Jogada invalida: {msg_erro}");
                continue;
            }
        }
    }
}
