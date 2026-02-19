// tabuleiro.rs

//! Define o tabuleiro e toda lógica do jogo: jogadas, vitória e empate

use std::fmt::Display;
use std::ops;

use crate::{Jogada, Simbolo};

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

/// O resultado possível após uma jogada
pub enum ResultadoJogo {
    /// O jogador completou uma combinação vencedora
    Vitoria(Simbolo),

    /// Todas as 9 céluals foram preenchidas sem vencedor
    Empate,

    /// O jogo continua normalmente
    EmAndamento,
}

/// O tabuleiro 3x3, representado por uma sequência plana de 9 células
///
/// Internamente usa índices de 0 a 8, mas expõe uma interface com posições de 1 a 9 (via `Index<usize>`) para corresponder ao que o usuário digita
pub struct Tabuleiro {
    celulas: [Celula; 9],

    /// Símbolo do último jogador a fazer uma jogada válida
    /// Usado para determinar de quem é a vez no turno seguinte
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

    /// Tenta aplicar uma jogada no tabuleiro
    /// 
    /// Retorna `Err` se a posição já estiver ocupada
    /// Retorna `Ok(ResultadoJogo)` indicando o estado do jogo após a jogada
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

    /// Verifica se alguma das 8 combinações possíveis de vitória foi feita
    pub fn verificar_vitoria(&self) -> bool {
        for combinacao in COMBINACOES_VITORIA {
            let simbolo_a = self[combinacao[0]].simbolo;
            let simbolo_b = self[combinacao[1]].simbolo;
            let simbolo_c = self[combinacao[2]].simbolo;

            // Todos os 3 precisam ser Some e ter o mesmo valor
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

/// Uma posição no tabuleiro representada de duas formas complementares:
/// `amigavel` é o número que o usuário vê/digita (1 a 9).
/// `linha` e `coluna` são as coordenadas internas (0 a 2) derivadas dele
#[derive(Clone, Copy)]
pub struct Posicao {
    pub amigavel: usize,
    _linha: usize,
    coluna: usize,
}

impl Posicao {
    pub fn new(amigavel: usize) -> Self {
        // Converte de base 1 (usuáiro) para base 0 (array) antes de calcular
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

/// Uma célula do tabuleiro
/// Guarda sua posição e o símbolo que a ocupada, se houver algum
pub struct Celula {
    pub posicao: Posicao,
    pub simbolo: Option<Simbolo>,
}

impl Celula {
    fn new(posicao: Posicao) -> Self {
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
