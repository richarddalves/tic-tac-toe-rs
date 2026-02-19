// jogador/simbolo.rs

//! Define o símbolo usado por cada jogador no tabuleiro: X ou O

use std::fmt::Display;

/// Os dois símbolos possíveis no jogo da velha
///
/// Derica `Copy` para que seja possível ser passado por valor livremente sem precisar de clone ou referências
/// Derica `PartialEq` para que seja possível fazer comparações de igualdade entre os símbolos nas diferentes células e saber o jogador atual
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Simbolo {
    X,
    O,
}

impl Display for Simbolo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
}
