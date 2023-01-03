/// Game logic and functions of command line Tic-Tac-Toe combined with a very basic AI.
///
/// The basic sceleton was taken from <https://brandonio21.com/building-tic-tac-toe-in-rust-rustic_tac_toe/> and 
/// all credit for the idea goes to the *original* author.
/// This implementation was created by first following along the free tutorial and then
/// implementing several additions and re-implementing several different parsts. 
/// Among others, the original implementation was severly rewritten, most noteable changes include:
///
/// - simplified at several steps
/// - made more rusty
/// - diagonal checking was added to the AI logic implementation
/// - implementation was changed from the follow-along tutorial to a more sophisticated setup,
/// several changes to the implementation
/// - test added
/// - added different difficulties

/// All Tic-Tac-Toe logic is contained here.



mod gui;
pub use crate::gui::*;

pub mod tic_tac_toe {

    
    #[derive(PartialEq)]
    pub enum Field {
        Cross,
        Circle,
        Free,
    }
    #[derive(Clone, Copy, Debug)]
    pub enum Message {
        Number(usize),
        NewGame
    }

    /// This module contains all checks performed in the game, including:
    ///
    /// - check if the game is finished
    /// - check if the game ended in a stale mate
    /// - checks if the player is close to winning (for the AI to prevent it)
    pub mod check_functions {
        use super::Field;
        /// Check if we have three crosses or circels in a row, column or diagonal
        /// Which row, column or diagonal is checked is controlled by the start index and the step
        /// size.
        fn check_for_three(board: &[Field], index: usize, step_size: usize) -> (bool, Field) {
            if board[index] == board[index+2*step_size] && board[index+step_size] == board[index+2*step_size] {
                 match board[index] {
                    Field::Cross => (true, Field::Cross),
                    Field::Circle => (true, Field::Circle),
                    Field::Free => (false, Field::Free),
                }
            }
            else {
                (false, Field::Free)
            }
        }
        
        /// Check if game is over
        #[must_use]
        pub fn someone_has_won(board: &[Field]) -> (bool, Field) {
            for i in [0, 3, 6] {
                let return_value = check_for_three(board, i, 1);
                if return_value.0 {
                    return return_value
                }
            }
            for i in [0, 1, 2] {
                let return_value = check_for_three(board, i, 3);
                if return_value.0 {
                    return return_value
                }
            }
            let return_value = check_for_three(board, 0, 4);
            if return_value.0 {
                return return_value
            }
            let return_value = check_for_three(board, 2, 2);
            if return_value.0 {
                return return_value
            }
            (false, Field::Free)
        }

        /// Check for remis.
        #[must_use]
        pub fn cats_game(board: &[Field]) -> bool {
            // for item in &(reference) iterator
            for i in board {
                if *i == Field::Free {
                    return false
                }
            }
            true
        }

        /// Check if player has two out of three already and return index of missing value to
        /// prevent player win.
        #[must_use]
        pub fn check_for_two(board: &[Field], col: bool) -> (bool, usize) {
            for i in 0..3 {
                let mut count = 0;
                let mut free_cell_index = 0;
                for j in 0..3 {
                    let index = if col {
                         i + 3*j
                    }
                    else { j + 3*i};
                    if board[index] == Field::Free { free_cell_index = index; }
                    else { count += 1; }
                }
                if count == 2 { return (true, free_cell_index) }

            }
            (false, 0)
        }

        #[must_use]
        pub fn check_for_diagonals(board: &[Field]) -> (bool, usize)  {
            for i in [[2, 4, 6], [0, 4, 8]] {
                let mut diagonal_count = 0;
                let mut free_cell = 0;
                for j in i {
                    if board[j] == Field::Free {
                        free_cell = j;
                    }
                    else {
                        diagonal_count += 1;
                    }
                }
                if diagonal_count == 2 {
                    return (true, free_cell)
                }
            }
            (false, 0)
        }
    }


    /// This module contains the entire AI logic.
    mod ai_functions {
        use super::Field;
        extern crate rand;
        use rand::Rng;
        use rand::distributions::Uniform;
        use super::check_functions::check_for_two;
        use super::check_functions::check_for_diagonals;       
        use fltk::prelude::WidgetExt;
        /// Determine the next move of the AI - either a good calculated move or a random
        /// placement.
        pub fn ai_move(board: &mut Vec<Field>, button_vectors: &mut Vec<&mut crate::gui::MyButton>, difficulty: i32) {
            let mut random: bool = false;
            let mut rng = rand::thread_rng();
            if difficulty == 0 {
                random = rng.gen();
            }
            else  if difficulty == 1{
                let random_values = Uniform::new_inclusive(1, 3); 
                let mut roll_die = rand::thread_rng().sample_iter(&random_values);
                if roll_die.next().unwrap() > 2 {
                        random = true;
                }
            }
            else {
                let random_values = Uniform::new_inclusive(1, 12); 
                let mut roll_die = rand::thread_rng().sample_iter(&random_values);
                if roll_die.next().unwrap() == 12 {
                    random = true;
                }
            }
            if random {
                // random for the way
                let mut num: usize = rng.gen_range(0..9);
                while board[num] == Field::Cross || board[num] == Field::Circle {
                    num = rng.gen_range(0..9);
                }
                board[num] = Field::Circle;
                button_vectors[num].set_label("O");
            }
            else {
                // Do a good move. Look for a spot thas has an x adjacent to it. Prioritize the middle.
                if board[4] != Field::Cross && board[4] != Field::Circle {
                    board[4] = Field::Circle;
                    button_vectors[4].set_label("O");

                }
                else {
                    // check if there are any 2Xs in a row
                    let two_in_rows = check_for_two(board, false);
                    if two_in_rows.0 {
                        board[two_in_rows.1] = Field::Circle;
                        button_vectors[two_in_rows.1].set_label("O");
                        return
                    }
                    let two_in_cols = check_for_two(board, true);
                    if two_in_cols.0 {
                        board[two_in_cols.1] = Field::Circle;
                        button_vectors[two_in_cols.1].set_label("O");
                        return
                    }
                    let two_in_diags = check_for_diagonals(board);
                    if two_in_diags.0 {
                        board[two_in_diags.1] = Field::Circle;
                        button_vectors[two_in_diags.1].set_label("O");
                        return
                    }
                    let mut empty_spot = 10;
                    for i in 0..9 {
                        if board[i] == Field::Circle {
                            let new_spot = get_available_adjacent(board, i);
                            if new_spot < board.len() {
                                board[new_spot] = Field::Circle;
                                button_vectors[new_spot].set_label("O");
                                return;
                            }
                        }
                        else if board[i] != Field::Cross {
                            empty_spot = i;
                        }
                    }
                    board[empty_spot] = Field::Circle;
                    button_vectors[empty_spot].set_label("O");

                }
            }
        }

        /// Check which spot to choose for AI move if player is not close to winning.
        fn get_available_adjacent(board: &[Field], spot: usize) -> usize {
            if spot == 4 {
                let next_spots = vec![2, 0, 6, 8];
                for field in &next_spots {
                    if board[*field] == Field::Free {
                        return next_spots[*field];
                    }
                }
            }
            if spot + 3 < board.len() && board[spot + 3] != Field::Cross && board[spot + 3] != Field::Circle {
                return spot + 3;
            }
            else if (spot - 2) > 0 && board[spot - 3] != Field::Cross && board[spot - 3] != Field::Circle {
                return spot - 3;
            }
            else if ((spot + 1) / 3) == (spot / 3) && spot + 1 < board.len() {
                if board[spot + 1] != Field::Cross && board[spot + 1] != Field::Circle {
                    return spot + 1;
                }
            }
            else if ((spot -1) / 3) == (spot / 3) && board[spot - 1] != Field::Cross && board[spot - 1] != Field::Circle {
                    return spot - 1;
                }
            for (i, cell) in board.iter().enumerate() {
                if *cell != Field::Cross && *cell != Field::Circle {
                    return i;
                }
            }
            10
        }
    }

    /// This module contains the main loop of the game and player interaction.
    pub mod game {
        use super::{Field, Message, check_functions};
        use super::check_functions::{cats_game, someone_has_won};
        use super::ai_functions::ai_move;
        use fltk::app;
        use fltk::dialog;
        pub use crate::gui::*;
        /// Print board to command line
        fn _print_board(board: &[Field]) {
            println!("-------------");
            print!("| ");
            for (i, cell) in board.iter().enumerate() {
                match cell {
                    Field::Free => print!("  | "),
                    Field::Circle => print!("O | "),
                    Field::Cross => print!("X | "),
                }
                if i > 0 && (i + 1) % 3 == 0 {
                    println!();
                    if i < 8 {
                        println!("-------------");
                        print!("| ");
                    }
                }
            }
            println!("-------------");
        }

        /// Control game loop and in-game player interaction.
        fn game_loop() -> i32 {
            use fltk::{enums::Color, group::{Pack, PackType}, prelude::*,window::Window,};            
            // create gui
            let app = app::App::default().with_scheme(app::Scheme::Gleam);
            let win_w = 600;
            let win_h = 600;

            let mut wind = Window::default()
                .with_label("FLTK Tic-Tac-Toe")
                .with_size(win_w, win_h)
                .center_screen()
                .with_label("Tic-Tac-Toe");
            wind.set_color(Color::Light3);

            // create board
            let vertical_pack = Pack::new(0, 0, win_w, win_h, "");

            let mut horizontal_pack = Pack::new(0, 0, win_w, 200, "");
            let mut but7 = MyButton::new();
            let mut but8 = MyButton::new();
            let mut but9 = MyButton::new();
            horizontal_pack.end();
            horizontal_pack.set_type(PackType::Horizontal);

            let mut horizontal_pack = Pack::new(0, 0, win_w, 200, "");
            let mut but4 = MyButton::new();
            let mut but5 = MyButton::new();
            let mut but6 = MyButton::new();
            horizontal_pack.end();
            horizontal_pack.set_type(PackType::Horizontal);

            let mut horizontal_pack = Pack::new(0, 0, win_w, 200, "");
            let mut but1 = MyButton::new();
            let mut but2 = MyButton::new();
            let mut but3 = MyButton::new();

            horizontal_pack.end();
            horizontal_pack.set_type(PackType::Horizontal);


            vertical_pack.end();

            wind.make_resizable(false);
            wind.end();
            wind.show();

            app::set_focus(&*but1); // maybe remove this later

            let but_vectors = vec![
                &mut but1, &mut but2, &mut but3, &mut but4, &mut but5, &mut but6, &mut but7, &mut but8, &mut but9, 
            ];

            let (s, r) = app::channel::<Message>();
            for (cell_number, but) in but_vectors.into_iter().enumerate() {
                but.emit(s, Message::Number(cell_number));
            }
            let mut but_vectors = vec![
                &mut but1, &mut but2, &mut but3, &mut but4, &mut but5, &mut but6, &mut but7, &mut but8, &mut but9, 
            ];
            let mut board = vec![];
            for _ in 0..9 {
                let v = Field::Free;
                board.push(v);
            }
            let mut results = check_functions::someone_has_won(&board);
            let last_turn = 5;
            let mut turn = 1;

            let center = ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32);
            let difficulty_message = "Choose difficulty".to_string();
            let difficulty = dialog::choice(center.0 - 200, center.1 - 100, &difficulty_message, "Easy", "Challenge", "Hard");
            while app.wait() {
                if cats_game(&board) {
                    println!("Cats game! Remis!");
                    break;
                }
                // Step two, ask where the user wants to place their cross
                if let Some(val) = r.recv() {
                    match val {
                        Message::Number(num) => {
                            let answer = num;
                            match board.get(answer){
                                Some(Field::Cross) => {dialog::alert(center.0 - 200, center.1 - 100, "You have already taken this field!"); continue;},
                                Some(Field::Circle) => {dialog::alert(center.0 - 200, center.1 - 100, "Your opponent has already taken this field!"); continue;},
                                Some(Field::Free) => {
                                    board[answer] = Field::Cross;
                                    but_vectors[answer].set_label("X");
                                },
                                None => (),
                            }
                        }
                        Message::NewGame => {
                            println!("Not implemented yet!");
                        }
                    }
                    results = someone_has_won(&board);
                    if results.0 {
                        break;
                    }
                    if turn < last_turn {
                        ai_move(&mut board, &mut but_vectors, difficulty);
                    }
                    results = someone_has_won(&board);
                    turn += 1;
                    if results.0 {
                        app.quit();
                    }
                }
            }
            let count;
            let winner_message = match results.1 {
                Field::Cross => {count = 1; "You have won"},
                Field::Circle => {count = -1; "AI has won"},
                Field::Free => {count = 0; "Noone"},
            };
            dialog::message(center.0 - 300, center.1 - 100, winner_message);
            count
        }


        /// Manages game statistics and start-end game-player interaction.
        pub fn main() {
            let mut stat_player = 0;
            let mut stat_ai = 0;
            let mut stat_remis = 0;
            let mut new_game: bool = true;
            let center = ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32);
            while new_game {
                let counter = game_loop();
                match counter {
                    1 => stat_player += 1,
                    -1 => stat_ai += 1,
                    0 => stat_remis += 1,
                    _ => {},
                }
                let status_message = format!("Current Status:\nPlayer won {} games\n AI won {} games\n {} cat games\nStart a new game? ", stat_player, stat_ai, stat_remis);
                let answer = dialog::choice(center.0 - 200, center.1 - 100, &status_message, "Yes", "No", "");
                match answer {
                        1 =>  new_game = false,
                        _ => continue,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tic_tac_toe::Field;
    use crate::tic_tac_toe::check_functions::someone_has_won;
    use crate::tic_tac_toe::check_functions::cats_game;
    use crate::tic_tac_toe::check_functions::check_for_two;
    use crate::tic_tac_toe::check_functions::check_for_diagonals;

    // someone_has_won utilizes check_for_three so we are not testing this function seperately
    // (each test of someone_has_one also tests check_for_three)
    #[test]
    fn test_someone_has_won_rows() {
        let board = vec![Field::Cross, Field::Cross, Field::Cross,Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Free];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_rows_second() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Cross, Field::Cross, Field::Cross, Field::Free, Field::Free, Field::Free];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_rows_third() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Cross, Field::Cross, Field::Cross];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_cols() {
        let board = vec![Field::Cross, Field::Free, Field::Free,Field::Cross, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_cols_second_col() {
        let board = vec![Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross, Field::Free];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_cols_third_col() {
        let board = vec![Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_diags() {
        let board = vec![Field::Cross, Field::Free, Field::Free,Field::Free, Field::Cross, Field::Free, Field::Free, Field::Free, Field::Cross,];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_someone_has_won_diags_other_side() {
        let board = vec![Field::Free, Field::Free, Field::Cross, Field::Free, Field::Cross, Field::Free, Field::Cross, Field::Free, Field::Free];
        let results = someone_has_won(&board);
        assert_eq!(true, results.0);
    }
    #[test]
    fn test_cats_game_happened() {
        let board = vec![Field::Circle, Field::Circle, Field::Cross, Field::Circle, Field::Cross, Field::Circle, Field::Cross, Field::Circle, Field::Circle];
        assert_eq!(true, cats_game(&board));
    }
    #[test]
    fn test_cats_game_has_not_happened() {
        let board = vec![Field::Circle, Field::Circle, Field::Cross, Field::Circle, Field::Cross, Field::Circle, Field::Cross, Field::Circle, Field::Free];
        assert_eq!(false, cats_game(&board));
    }
    #[test]
    fn test_check_for_two_rows() {
        let board = vec![Field::Cross, Field::Cross, Field::Free,Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Free];
        let results = check_for_two(&board, false);
        assert_eq!(true, results.0);
        assert_eq!(2, results.1);
    }
    #[test]
    fn test_check_for_two_rows_second() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Free];
        let results = check_for_two(&board, false);
        assert_eq!(true, results.0);
        assert_eq!(4, results.1);
    }
    #[test]
    fn test_check_for_two_rows_third() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Cross, Field::Cross, Field::Free];
        let results = check_for_two(&board, false);
        assert_eq!(true, results.0);
        assert_eq!(8, results.1);
    }
    #[test]
    fn test_check_for_two_cols() {
        let board = vec![Field::Cross, Field::Free, Field::Free,Field::Free, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free];
        let results = check_for_two(&board, true);
        assert_eq!(true, results.0);
        assert_eq!(3, results.1);
    }
    #[test]
    fn test_check_for_two_cols_second_col() {
        let board = vec![Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Free, Field::Free];
        let results = check_for_two(&board, true);
        assert_eq!(true, results.0);
        assert_eq!(7, results.1);
    }
    #[test]
    fn test_check_for_two_cols_third_col() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Free, Field::Cross];
        let results = check_for_two(&board, true);
        assert_eq!(true, results.0);
        assert_eq!(2, results.1)
    }
    #[test]
    fn test_check_for_diagonals() {
        let board = vec![Field::Cross, Field::Free, Field::Free,Field::Free, Field::Cross, Field::Free, Field::Free, Field::Free, Field::Free];
        let results = check_for_diagonals(&board);
        assert_eq!(true, results.0);
        assert_eq!(8, results.1);
    }
    #[test]
    fn test_check_for_diagonals_other_side() {
        let board = vec![Field::Free, Field::Free, Field::Free, Field::Free, Field::Cross, Field::Free, Field::Cross, Field::Free, Field::Free];
        let results = check_for_diagonals(&board);
        assert_eq!(true, results.0);
        assert_eq!(2, results.1)
    }
}
