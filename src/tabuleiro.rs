use std::fmt::Display;
use std::ops;

use crate::{Jogada, Simbolo};

const COMBINACOES_VITORIA: [[usize; 3]; 8] = [
    [1, 2, 3], // linha 1
    [4, 5, 6], // linha 2
    [7, 8, 9], // linha 3
    [1, 4, 7], // coluna 1
    [2, 5, 8], // coluna 2
    [3, 6, 9], // coluna 3
    [1, 5, 9], // diagonal principal
    [3, 5, 7], // diagonal secundária
];

pub enum ResultadoJogo {
    Vitoria(Simbolo),
    Empate,
    EmAndamento,
}

pub struct Tabuleiro {
    celulas: [Celula; 9],
    pub ultimo_a_jogar: Option<Simbolo>,
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
            ultimo_a_jogar: None,
            celulas_ocupadas: 0,
        }
    }

    pub fn jogar(&mut self, jogada: Jogada) -> Result<ResultadoJogo, &str> {
        let posicao_jogada = jogada.posicao.amigavel as usize;

        if let None = &self[posicao_jogada].simbolo {
            self[posicao_jogada] = Celula {
                posicao: self[posicao_jogada].posicao,
                simbolo: Some(jogada.simbolo),
            };

            self.ultimo_a_jogar = Some(jogada.simbolo);
            self.celulas_ocupadas += 1;

            if self.verificar_vitoria() {
                Ok(ResultadoJogo::Vitoria(jogada.simbolo))
            } else if self.celulas_ocupadas == 9 {
                Ok(ResultadoJogo::Empate)
            } else {
                Ok(ResultadoJogo::EmAndamento)
            }
        } else {
            Err("Essa posição já está ocupada")
        }
    }

    pub fn verificar_vitoria(&self) -> bool {
        for combinacao in COMBINACOES_VITORIA {
            let simbolo_a = self[combinacao[0]].simbolo;
            let simbolo_b = self[combinacao[1]].simbolo;
            let simbolo_c = self[combinacao[2]].simbolo;

            if simbolo_a.is_some() && simbolo_a == simbolo_b && simbolo_b == simbolo_c {
                return true;
            }
        }

        false
    }
}

impl ops::Index<usize> for Tabuleiro {
    type Output = Celula;

    fn index(&self, index: usize) -> &Self::Output {
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
            match celula.posicao.coluna {
                0 | 1 => write!(f, "{}  ", celula)?,
                2 => writeln!(f, "{}", celula)?,
                _ => panic!("Tem mais colunas do que deveria"),
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Posicao {
    pub amigavel: usize,
    _linha: usize,
    coluna: usize,
}

impl Posicao {
    pub fn new(amigavel: usize) -> Self {
        let (linha, coluna) = {
            let linha = (amigavel - 1) / 3;
            let coluna = (amigavel - 1) % 3;

            (linha, coluna)
        };

        Self {
            amigavel,
            _linha: linha,
            coluna,
        }
    }
}

pub struct Celula {
    pub posicao: Posicao,
    pub simbolo: Option<Simbolo>,
}

impl Celula {
    fn new(posicao: Posicao) -> Self {
        Self {
            posicao,
            simbolo: None,
        }
    }
}

impl Display for Celula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.simbolo {
            Some(simbolo) => write!(f, "{}", simbolo),
            None => write!(f, "_"),
        }
    }
}
