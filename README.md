# Rust Guessing Game

A simple, interactive number guessing game written in Rust. This project is a great starting point for beginners to learn about core Rust concepts.

The program generates a secret random number between 1 and 100, and the player must guess what it is. The program provides feedback of "Too high!" or "Too low!" until the player guesses the correct number.

## Features

*   Generates a random number for the user to guess.
*   Reads user input from the command line.
*   Provides feedback on the user's guess.
*   Handles invalid (non-numeric) input gracefully.
*   Demonstrates the use of an external crate (`rand`).

## Prerequisites

To build and run this project, you need to have the Rust toolchain installed. If you don't have it, you can install it from the official site: [https://rustup.rs/](https://rustup.rs/)

## Getting Started

1.  **Clone the repository or create the files locally:**
    You can either clone this repository or manually create the files (`Cargo.toml`, `src/main.rs`) as shown below.

2.  **Build the project:**
    This command compiles your code and its dependencies into an executable binary.
    ```sh
    cargo build
    ```
    The executable will be located at `target/debug/rust-guessing-game`.

3.  **Run the project:**
    This command will both compile and run the program in one step.
    ```sh
    cargo run
    ```
    The game will start, and you can begin guessing!

    To run the optimized release version (which compiles slower but runs faster):
    ```sh
    cargo run --release
    ```

## Project Structure
