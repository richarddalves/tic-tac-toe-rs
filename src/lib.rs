// lib.rs

//! Biblioteca principal do jogo da velha
//! 
//! Expõe os tipos públicos e a função `run`, que contém o loop principal da partida

mod jogador;
mod tabuleiro;

use std::io::{self, Write};

pub use jogador::{Jogador, Simbolo};
pub use tabuleiro::{Posicao, ResultadoJogo, Tabuleiro};

/// Representa uma jogada: qual símbolo será marcado e em qual posíção
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

/// Inicia e controla o loop principal da partida
/// 
/// Alterna o turno entre os dois jogadores, lê a posição desejada e aplica a jogada e verifica o resultado até o jogo terminar
pub fn run(jogador1: Jogador, jogador2: Jogador, mut mesa: Tabuleiro) {
    let mut input_buffer = String::new();
    println!("=== JOGO DA VELHA ===");
    println!("{mesa}");

    loop {
        // Determina o próximo jogador comparando com o símbolo do último que jogou. Quem jogou por último não pode jogar de novo
        let proximo_a_jogar = match mesa.ultimo_a_jogar {
            Some(simbolo) => {
                if simbolo == jogador1.simbolo {
                    &jogador2
                } else {
                    &jogador1
                }
            }

            None => &jogador1, // Primeira jogada começa sempre com jogador1
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
