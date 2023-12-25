# Rock-Paper-Scissors-Lizard-Spock Game

This is an implementation of the extended version of the classic game Rock-Paper-Scissors, including Lizard and Spock, written in Rust. The game allows a player to compete against the computer in a series of rounds.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and playing purposes.

### Prerequisites

To run this program, you will need Rust installed on your system. If you don't have Rust installed, you can download it from [the official Rust website](https://www.rust-lang.org/learn/get-started).

### Installing

1. Clone the repository or download the source code.
2. Navigate to the directory containing the source code.
3. Run the program using Cargo, Rust's package manager and build system.

    ```bash
    cargo run
    ```

## Gameplay

The game is a command-line application. At the start of each round, the player is prompted to choose between Rock (1), Paper (2), Scissors (3), Lizard (4), or Spock (5). The computer will randomly choose one of the options. The game then displays the outcome of the round.

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

To end the game, enter `0` when prompted for your choice. The game will then exit the loop and end.

## Scoring

The game keeps track of the number of rounds played, as well as the player's and computer's scores. After each round, it displays the current game count and the scores.

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.

## Author

- Your Name

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Hat tip to anyone whose code was used
- Inspiration
- etc
