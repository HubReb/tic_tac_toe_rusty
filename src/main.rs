#![warn(clippy::all, clippy::pedantic)]
/// A simple command line Tic-Tac-Toe game.
///
/// A command line Tic-Tac-Toe game with a simple AI that was inspired by '<https://brandonio21.com/building-tic-tac-toe-in-rust-rustic_tac_toe/>' ; credit for the idea and basic AI logic goes to the original author.
///
/// # Example
/// 
/// // start game
/// `./rustic_tac_toe`
use tic_tac_toe::tic_tac_toe::game::main as game_loop;

fn main() {
    game_loop();
}
