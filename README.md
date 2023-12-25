# Rock-Paper-Scissors-Lizard-Spock Game

As a beginner in Rust programming, I, Swadha, have embarked on creating this personal hobby project. It's an implementation of the extended version of the classic game Rock-Paper-Scissors, which includes Lizard and Spock. This game, inspired by the popular TV show "The Big Bang Theory," offers a playful and innovative twist on the traditional Rock-Paper-Scissors game and serves as a platform for me to learn and grow in Rust programming.

## Getting Started

These instructions will guide you in getting a copy of the project up and running on your local machine for development and entertainment purposes.

### Prerequisites

To run this program, you will need Rust installed on your system. If Rust is not already installed, you can download it from [the official Rust website](https://www.rust-lang.org/learn/get-started).

### Installing

1. Clone the repository or download the source code.
2. Navigate to the directory containing the source code.
3. Use Cargo, the Rust package manager and build system, to run the program.

    ```bash
    cargo run
    ```

## Gameplay

The game operates as a command-line application. At the start of each round, you will be prompted to choose between Rock (1), Paper (2), Scissors (3), Lizard (4), or Spock (5). The computer will then randomly select one of these options, and the game will display the outcome of the round.

### Rules

- Rock crushes Scissors
- Scissors cuts Paper
- Paper covers Rock
- Rock crushes Lizard
- Lizard poisons Spock
- Spock smashes Scissors
- Scissors decapitates Lizard
- Lizard eats Paper
- Paper disproves Spock
- Spock vaporizes Rock

### Ending the Game

To end the game, enter `0` when prompted for your choice. The game will then exit the loop and conclude.

## Scoring

The game keeps track of the number of rounds played, as well as the scores of both the player and the computer. After each round, it displays the current game count and the scores.

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
