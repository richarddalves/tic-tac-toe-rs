// jogador/simbolo.rs

use std::fmt::Display;

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
