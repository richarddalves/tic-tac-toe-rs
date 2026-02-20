// src/main.rs

//! Ponto de entrada do programa
//!
//! Coleta informações dos jogadores via terminal e inicia a partida

use std::io::{self, Write};
use ttt::{ErroJogo, Jogador, Partida, ResultadoJogo, Simbolo};

fn main() {
    println!("========== Jogo da Velha ==========");

    let jogador1 = pedir_nome("Insira o nome do primeiro jogador: ");
    let simbolo1 = pedir_simbolo("Escolha o seu simbolo [X ou O]: ");
    let jogador2 = pedir_nome("Insira o nome do segundo jogador: ");

    let (jogador1, jogador2) = Jogador::iniciar(jogador1, jogador2, simbolo1);

    let partida = Partida::new(jogador1, jogador2);

    run(partida);
}

fn run(mut partida: Partida) {
    loop {
        println!("{}", &partida.tabuleiro);

        let posicao = pedir_posicao(&format!(
            "{}, insira sua próxima jogada (1-9): ",
            partida.jogador_atual()
        ));

        match partida.jogar(posicao) {
            Ok(ResultadoJogo::Vitoria) => {
                println!("{}", &partida.tabuleiro);
                println!("========= {} ganhou!!! =========", partida.jogador_atual());
                break;
            }

            Ok(ResultadoJogo::Empate) => {
                println!("{}", &partida.tabuleiro);
                println!("========= EMPATE!!! =========");
                break;
            }

            Ok(ResultadoJogo::EmAndamento) => continue,

            Err(e) => eprintln!("{e}"),
        }
    }
}

/// Lê uma linha do terminal e retorna o conteúdo sem espaços nas bordas
fn pedir_nome(msg: &str) -> String {
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

/// Lê o símbolo escolhido pelo jogador1 (X ou O)
/// repetindo até receber uma entrada válida
fn pedir_simbolo(msg: &str) -> Simbolo {
    let mut input_buffer = String::new();

    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();
        input_buffer = input_buffer.trim().to_lowercase();

        if input_buffer == "x" {
            return Simbolo::X;
        } else if input_buffer == "o" {
            return Simbolo::O;
        } else {
            continue; // entrada inválida, o loop continua
        }
    }
}

fn pedir_posicao(msg: &str) -> usize {
    let mut input_buffer = String::new();

    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();

        match input_buffer.trim().parse::<usize>() {
            Ok(posicao) if posicao >= 1 && posicao <= 9 => return posicao,
            _ => {
                eprintln!("{}", ErroJogo::PosicaoInvalida);
                continue;
            }
        }
    }
}
