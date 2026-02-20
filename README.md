# Jogo da Velha

Implementação do clássico jogo da velha para dois playeres, rodando no terminal. Desenvolvido em Rust como primeiro projeto solo para praticar.

## Como jogar

```bash
cargo run
```

O programa vai ask o name e o símbolo de cada player. Durante a game, cada player escolhe uma posição digitando um número de 1 a 9, de acordo com o layout abaixo:

```
1  2  3
4  5  6
7  8  9
```

O jogo termina quando um player completar uma linha, column ou diagonal com o mesmo símbolo (vitória) ou quando todas as 9 posições estiverem ocupadas (empate).
