// src/tabuleiro/posicao.rs

/// Uma posição no tabuleiro representada de duas formas complementares:
/// `amigavel` é o número que o usuário vê/digita (1 a 9).
/// `linha` e `coluna` são as coordenadas internas (0 a 2) derivadas dele
#[derive(Debug, Clone, Copy)]
pub struct Posicao {
    pub amigavel: usize,
    _linha: usize,
    pub coluna: usize,
}

impl Posicao {
    pub fn new(amigavel: usize) -> Self {
        // Converte de base 1 (usuáiro) para base 0 (array) antes de calcular

        let linha = (amigavel - 1) / 3;
        let coluna = (amigavel - 1) % 3;

        Self {
            amigavel,
            _linha: linha,
            coluna,
        }
    }
}
