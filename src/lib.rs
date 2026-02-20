// src/lib.rs

//! Biblioteca principal do jogo da velha
//!
//! Expõe os tipos públicos e a função `run`, que contém o loop principal da partida

mod erro;
mod jogador;
mod partida;
mod tabuleiro;

pub use erro::ErroJogo;
pub use jogador::{Jogador, Simbolo};
pub use partida::{Partida, ResultadoJogo};
pub use tabuleiro::{Posicao, Tabuleiro};
