// src/partida.rs

use crate::{Jogador, Simbolo, Tabuleiro, erro::ErroJogo};

/// O resultado possível após uma jogada
#[derive(PartialEq, Eq)]
pub enum ResultadoJogo {
    /// O jogador completou uma combinação vencedora
    Vitoria,

    /// Todas as 9 céluals foram preenchidas sem vencedor
    Empate,

    /// O jogo continua normalmente
    EmAndamento,
}

/// As 8 combinaçõs possíveis que resultam em vitória
/// 3 linhas, 3 colunas, 2 diagonais
///
/// Cada sub-array usa as posições "amigáveis" (1-9) que o usuário vê
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

#[derive(Debug)]
pub struct Partida {
    pub tabuleiro: Tabuleiro,
    jogador1: Jogador,
    jogador2: Jogador,
    turno_atual: Simbolo,
}

impl Partida {
    pub fn new(jogador1: Jogador, jogador2: Jogador) -> Self {
        let tabuleiro = Tabuleiro::new();
        let turno_atual = jogador1.simbolo;

        Self {
            tabuleiro,
            jogador1,
            jogador2,
            turno_atual,
        }
    }

    pub fn jogador_atual(&self) -> &Jogador {
        if self.turno_atual == self.jogador1.simbolo {
            &self.jogador1
        } else {
            &self.jogador2
        }
    }

    fn verificar_vitoria(&self) -> bool {
        for combinacao in COMBINACOES_VITORIA {
            let simbolo_a = &self.tabuleiro[combinacao[0]].simbolo;
            let simbolo_b = &self.tabuleiro[combinacao[1]].simbolo;
            let simbolo_c = &self.tabuleiro[combinacao[2]].simbolo;

            if simbolo_a.is_some() && simbolo_a == simbolo_b && simbolo_b == simbolo_c {
                return true;
            }
        }

        false
    }

    pub fn jogar(&mut self, posicao: usize) -> Result<ResultadoJogo, ErroJogo> {
        self.tabuleiro.marcar(posicao, self.turno_atual)?;

        if self.verificar_vitoria() {
            Ok(ResultadoJogo::Vitoria)
        } else if self.tabuleiro.esta_cheio() {
            Ok(ResultadoJogo::Empate)
        } else {
            self.mudar_turno();
            Ok(ResultadoJogo::EmAndamento)
        }
    }

    fn mudar_turno(&mut self) {
        self.turno_atual = if self.turno_atual == Simbolo::X {
            Simbolo::O
        } else {
            Simbolo::X
        };
    }
}
