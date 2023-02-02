# tic_tac_toe_rusty

A simple Tic-Tac-Toe game with a basic GUI.

This was inspired by the command line Tic-Tac-Toe game with a simple AI that is introduced in [this free tutorial](https://brandonio21.com/building-tic-tac-toe-in-rust-rustic_tac_toe/) by [Brandon Milton](https://github.com/brandonio21).
All credit for the idea goes to the *original author*.

This implementation was created by first roughly following the free tutorial, then re-implementing with thorough changes and adding several additions, such as different difficulties, score keeper and GUI. 
Most noteable changes include:

 * simplified at several steps
 * made more rusty
 * added diagonal checking to the AI logic implementation and altered next action choice of AI accordingly
 * implementation was severly changed from the follow-along tutorial to a more sophisticated setup (e. g. one file -> lib, gui, main)
 * several changes to the implementation (AI logic, checks, ...)
 * tests added
 * added different difficulties

## Installation

Simply run `cargo build --release` followed by running the compiled bynary (./target/release/rustic_tac_toe)

## State of the Code
This small project was done to improve my skills in Rust. The code was written in Dec, 2021, and reflects my skills at that time.
