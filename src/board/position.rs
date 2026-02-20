// src/board/position.rs

/// A position on the board represented in two complementary ways:
/// `value` is the number the user sees/types (1 to 9).
/// `_row` and `column` are the internal coordinates (0 to 2) derived from it.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub value: usize,
    _row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(value: usize) -> Self {
        // Converts from 1-based (user-facing) to 0-based (array index)
        let row = (value - 1) / 3;
        let column = (value - 1) % 3;

        Self {
            value,
            _row: row,
            column,
        }
    }
}
