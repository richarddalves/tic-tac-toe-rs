// src/tabuleiro/celula.rs

use super::Posicao;
use super::Simbolo;

use std::fmt::Display;

/// Uma célula do tabuleiro
/// Guarda sua posição e o símbolo que a ocupada, se houver algum
#[derive(Debug)]
pub struct Celula {
    pub posicao: Posicao,
    pub simbolo: Option<Simbolo>,
}

impl Celula {
    pub fn new(posicao: Posicao) -> Self {
        Self {
            posicao,
            simbolo: None, // Célula começa vazia
        }
    }
}

impl Display for Celula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.simbolo {
            Some(simbolo) => write!(f, "{}", simbolo),
            None => write!(f, "_"), // Célula vazia exibe underscore em vez de algum símbolo
        }
    }
}
