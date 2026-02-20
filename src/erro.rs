// src/erro.rs

use std::fmt::Display;

#[derive(Debug)]
pub enum ErroJogo {
    PosicaoOcupada,
    PosicaoInvalida, // número fora do range 1-9
}

impl Display for ErroJogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErroJogo::PosicaoInvalida => write!(
                f,
                "Erro: Você deve digitar uma posição entre 1 e 9. Tente novamente."
            ),
            ErroJogo::PosicaoOcupada => {
                write!(f, "Essa posição já está ocupada. Tente outra posição.")
            }
        }
    }
}
