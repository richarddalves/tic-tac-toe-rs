# Tic-Tac-Toe

Implementation of the classic tic-tac-toe game for two players, running in the terminal. Developed in Rust as a first solo project for practice.

## How to play
```bash
cargo run
```

The program will ask for each player's name and symbol. During the game, each player chooses a position by typing a number from 1 to 9, according to the layout below:
```
1  2  3
4  5  6
7  8  9
```

The game ends when a player completes a row, column, or diagonal with the same symbol (victory), or when all 9 positions are occupied (draw).