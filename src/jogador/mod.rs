// src/jogador/mod.rs

//! Define o jogador e a lógica de inicialização de uma partida

mod simbolo;
use std::fmt::Display;

pub use simbolo::Simbolo;

/// Um participante do jogo, identificado pelo nome e pelo símbolo que usa.
#[derive(Debug, PartialEq, Eq)]
pub struct Jogador {
    pub nome: String,
    pub simbolo: Simbolo,
}

impl Jogador {
    fn new(nome: String, simbolo: Simbolo) -> Self {
        Self { nome, simbolo }
    }

    /// Inicializa os dois jogadores para uma partida
    ///
    /// O jogador 1 escolhe seu símbolo
    /// O símbolo do jogador 2 é determinado automaticamente como o oposto
    pub fn iniciar(nome1: String, nome2: String, simbolo1: Simbolo) -> (Self, Self) {
        let simbolo2 = if simbolo1 == Simbolo::X {
            Simbolo::O
        } else {
            Simbolo::X
        };

        let jogador1 = Self::new(nome1, simbolo1);
        let jogador2 = Self::new(nome2, simbolo2);

        (jogador1, jogador2)
    }
}

impl Display for Jogador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nome)
    }
}
