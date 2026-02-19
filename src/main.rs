// main.rs

//! Ponto de entrada do programa
//! 
//! Coleta informações dos jogadores via terminal e inicia a partida

use std::io::{self, Write};

use jogo_da_velha::{Jogador, Simbolo, Tabuleiro, run};

fn main() {
    let mesa = Tabuleiro::new();

    println!("========== Jogo da Velha ==========");

    let jogador1 = pedir_nome("Insira o nome do primeiro jogador: ");
    let simbolo1 = pedir_simbolo("Escolha o seu simbolo [X ou O]: ");
    let jogador2 = pedir_nome("Insira o nome do segundo jogador: ");

    let (jogador1, jogador2) = Jogador::iniciar(jogador1, jogador2, simbolo1);

    run(jogador1, jogador2, mesa);
}

/// Lê uma linha do terminal e retorna o conteúdo sem espaços nas bordas
fn pedir_nome(msg: &str) -> String {
    let mut input_buffer = String::new();

    print!("{msg}");
    io::stdout().flush().unwrap();

    input_buffer.clear();
    io::stdin().read_line(&mut input_buffer).unwrap();

    input_buffer.trim().to_string()
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
