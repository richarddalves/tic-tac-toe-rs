// src/tabuleiro/mod.rs

//! Define o tabuleiro e toda lógica do jogo: jogadas, vitória e empate

mod celula;
mod posicao;

pub use celula::Celula;
pub use posicao::Posicao;

use std::fmt::Display;
use std::ops;

use crate::{Simbolo, erro::ErroJogo};

/// O tabuleiro 3x3, representado por uma sequência plana de 9 células
///
/// Internamente usa índices de 0 a 8, mas expõe uma interface com posições de 1 a 9 (via `Index<usize>`) para corresponder ao que o usuário digita

#[derive(Debug)]
pub struct Tabuleiro {
    celulas: [Celula; 9],
    celulas_ocupadas: u8,
}

impl Tabuleiro {
    pub fn new() -> Self {
        Self {
            celulas: [
                Celula::new(Posicao::new(1)),
                Celula::new(Posicao::new(2)),
                Celula::new(Posicao::new(3)),
                Celula::new(Posicao::new(4)),
                Celula::new(Posicao::new(5)),
                Celula::new(Posicao::new(6)),
                Celula::new(Posicao::new(7)),
                Celula::new(Posicao::new(8)),
                Celula::new(Posicao::new(9)),
            ],
            celulas_ocupadas: 0,
        }
    }

    pub fn marcar(&mut self, posicao: usize, simbolo: Simbolo) -> Result<(), ErroJogo> {
        if posicao < 1 || posicao > 9 {
            return Err(ErroJogo::PosicaoInvalida);
        }

        if let Some(_) = self[posicao].simbolo {
            return Err(ErroJogo::PosicaoOcupada);
        }

        self[posicao].simbolo = Some(simbolo);

        self.celulas_ocupadas += 1;

        Ok(())
    }

    pub fn esta_cheio(&self) -> bool {
        self.celulas_ocupadas == 9
    }
}

impl ops::Index<usize> for Tabuleiro {
    type Output = Celula;

    fn index(&self, index: usize) -> &Self::Output {
        // O usuário usa posições 1-9. O array interno usa 0-8
        self.celulas.get(index - 1).expect("Fora do range")
    }
}

impl ops::IndexMut<usize> for Tabuleiro {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.celulas.get_mut(index - 1).expect("Fora do range")
    }
}

impl Display for Tabuleiro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for celula in &self.celulas {
            // Usa a coluna da posição para saber quando quebrar a linha, evitando necessidade de um contador externo
            match celula.posicao.coluna {
                0 | 1 => write!(f, "{}  ", celula)?,
                2 => writeln!(f, "{}", celula)?,
                _ => panic!("Tem mais colunas do que deveria"),
            }
        }

        Ok(())
    }
}
