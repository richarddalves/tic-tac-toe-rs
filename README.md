# Jogo da Velha

Implementação do clássico jogo da velha para dois jogadores, rodando no terminal. Desenvolvido em Rust como primeiro projeto solo para praticar.

## Como jogar

```bash
cargo run
```

O programa vai pedir o nome e o símbolo de cada jogador. Durante a partida, cada jogador escolhe uma posição digitando um número de 1 a 9, de acordo com o layout abaixo:

```
1  2  3
4  5  6
7  8  9
```

O jogo termina quando um jogador completar uma linha, coluna ou diagonal com o mesmo símbolo (vitória) ou quando todas as 9 posições estiverem ocupadas (empate).

