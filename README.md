# Filler_rs

A Rust clone of the GamePigeon game Filler, created as a project for the [Boot.dev](https://www.boot.dev) Back-End Developer course.

# Installation

To player filler_rs:

1. Install the Rust toolchain with [rustup](https://rustup.rs/)
2. Clone this repo by running the command `git clone https://www.github.com/EMBradley/filler_rs`
3. Navigate into the repo with `cd filler_rs`
4. Run the game with `cargo run`

# Gameplay

Filler is a very simple game. Here's how it's played:

1. The game starts with a randomly generated grid of colored tiles. To begin, player one owns the bottom left tile, and player two owns the top right tile.
2. On each player's turn, they choose a color to change their tiles to, and they gain control of any unowned tiles of that color adjacent to the tiles they already control.
3. The game is over when all tiles are owned. The player with the most tiles wins.

![image of the beginning of a filler game](/images/example_1.png)

![image of the end of a filler game](/images/example_2.png)
