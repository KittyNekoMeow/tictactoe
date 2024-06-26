use crate::def::{board_new, Player};
use crate::engine::{if_random, run_engine, use_engine};
use std::io;
/// Print the board to the console
pub fn print_board(board: &[[u8; 3]; 3]) {
    // Makes the empty cells
    let mut cell1 = "|       ";
    let mut cell2 = "|       ";
    let mut cell3 = "|       |";
    let mut cell4 = "|       ";
    let mut cell5 = "|       ";
    let mut cell6 = "|       |";
    let mut cell7 = "|       ";
    let mut cell8 = "|       ";
    let mut cell9 = "|       |";

    // Goes through every vector to check if player X or O has played there
    // Changes the cell accordingly
    if board[0][0] == 1 {
        cell1 = "|   X   ";
    } if board[0][0] == 2 {
        cell1 = "|   O   ";
    } if board[0][1] == 1 {
        cell2 = "|   X   ";
    } if board[0][1] == 2 {
        cell2 = "|   O   ";
    } if board[0][2] == 1 {
        cell3 = "|   X   |";
    } if board[0][2] == 2 {
        cell3 = "|   O   |";
    } if board[1][0] == 1 {
        cell4 = "|   X   ";
    } if board[1][0] == 2 {
        cell4 = "|   O   ";
    } if board[1][1] == 1 {
        cell5 = "|   X   ";
    } if board[1][1] == 2 {
        cell5 = "|   O   ";
    } if board[1][2] == 1 {
        cell6 = "|   X   |";
    } if board[1][2] == 2 {
        cell6 = "|   O   |";
    } if board[2][0] == 1 {
        cell7 = "|   X   ";
    } if board[2][0] == 2 {
        cell7 = "|   O   ";
    } if board[2][1] == 1 {
        cell8 = "|   X   ";
    } if board[2][1] == 2 {
        cell8 = "|   O   ";
    } if board[2][2] == 1 {
        cell9 = "|   X   |";
    } if board[2][2] == 2 {
        cell9 = "|   O   |";
    }
        println!("{}{}{}", cell1, cell2, cell3);
        println!("{}{}{}", cell4, cell5, cell6);
        println!("{}{}{}", cell7, cell8, cell9);
    
}
/// Swap players
pub fn switch_player_turn(player: &Player) -> Player {
    match player {
        Player::PlayerOne => Player::PlayerTwo,
        Player::PlayerTwo => Player::PlayerOne,
    }
}
/// Receive player input
pub fn receive_input() -> String {
    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    // Calls receive input again if player inputed non allowed string
    if check_player_input(&player_input_clean) == false {
        return receive_input();
    }
    return player_input_clean;
}
/// Checks if input has already been received
pub fn if_input_exists(all_inputs: &Vec<String>, new_input: String) -> bool {
    for i in all_inputs {
        if i == &new_input {
            return true;
        }
    }
    return false;
}
/// Checks that player input is one of the allowed strings
pub fn check_player_input(player_input: &String) -> bool {
    match player_input.parse() {
        Ok(1..=9) => true,
        Err(_) => false,
        _ => false
    }
}
/// Pushes 1 or 0 to the cell corresponding to the player type
pub fn update_board_state(board: &mut [[u8; 3]; 3], player: &Player, player_input: &String) {
    if player == &Player::PlayerOne {
        match player_input.as_str() {
            "1" => board[0][0] = 1,
            "2" => board[0][1] = 1,
            "3" => board[0][2] = 1,
            "4" => board[1][0] = 1,
            "5" => board[1][1] = 1,
            "6" => board[1][2] = 1,
            "7" => board[2][0] = 1,
            "8" => board[2][1] = 1,
            "9" => board[2][2] = 1,
            _ => ()
        };
    } else {
        match player_input.as_str() {
            "1" => board[0][0] = 2,
            "2" => board[0][1] = 2,
            "3" => board[0][2] = 2,
            "4" => board[1][0] = 2,
            "5" => board[1][1] = 2,
            "6" => board[1][2] = 2,
            "7" => board[2][0] = 2,
            "8" => board[2][1] = 2,
            "9" => board[2][2] = 2,
            _ => ()
        };
    }
}
/// User chooses player by typeing either X or O
pub fn choose_player() -> Player {
    println!("To choose player please type X(first player) or O(second player)");
    let mut player_input = String::new().to_lowercase();
    io::stdin().read_line(&mut player_input).unwrap();
    let player_input_clean: String = player_input.trim().parse().unwrap();
    match player_input_clean.as_str() {
        "x" => Player::PlayerOne,
        "o" => Player::PlayerTwo,
        _ => choose_player()
    }
}
#[allow(unused_assignments)]
/// A function that checks if any of the winning positions are reached
pub fn did_win(board: &[[u8; 3]; 3], player: &Player) -> bool {
    let mut num = 0;
    
    if player == &Player::PlayerOne {
        num = 1;
    } else {
        num = 2;
    }
    if board[0][0] == num && board[0][1] == num && board[0][2] == num {
        return true;
    } else if board[1][0] == num && board[1][1] == num && board[1][2] == num {
        return true;
    } else if board[2][0] == num && board[2][1] == num && board[2][2] == num {
        return true;
    }  else if board[0][0] == num && board[1][0] == num && board[2][0] == num {
        return true;
    } else if board[0][1] == num && board[1][1] == num && board[2][1] == num {
        return true;
    } else if board[0][2] == num && board[1][2] == num && board[2][2] == num {
        return true;
    } else if board[0][0] == num && board[1][1] == num && board[2][2] == num {
        return true;
    } else if board[0][2] == num && board[1][1] == num && board[2][0] == num {
        return true;
    } else {
        return false;
    }
}
/// Function that combines all functions to make the program run
pub fn play_game() {
    // An if statement that lets user decide to play with the engine or not
    if use_engine() {
        play_game_with_engine();
    } else {
    let mut board = board_new();
    // Holds all input used to check if already have input and for the main loop condition
    let mut all_inputs: Vec<String> = Vec::new();
    // Player starts as X because X goes first
    let mut player = Player::PlayerOne;
    println!("X goes first");
    println!("To choose a square pick a number between 1 and 9");
    loop {
        // Loop breaks if board becomes full
        if all_inputs.len() == 9 {
            break;
        }
        print_board(&board);
        // Receive user input
        let mut player_input = receive_input();
        // Checks if cell is already filled if it is then call receive_input again
        while if_input_exists(&all_inputs, player_input.clone()) {
            player_input = receive_input();
        }
        // Update the board with the input
        update_board_state(&mut board, &player, &player_input);
        all_inputs.push(player_input.clone());
        // If win break loop
        if did_win(&board, &player) {
            break;
        }
        // Switch player turn
        player = switch_player_turn(&player);

    }
        // Prints the final board
        print_board(&board);
        // If win check which player and print the corresponding string
        if did_win(&board, &player) {
            if player == Player::PlayerOne {
                return println!("Good job on winning player X! Thanks for playing. Hope to see you around again.");
            } else {
                return println!("Good job on winning player O! Thanks for playing. Hope to see you around again.");
            }
        }
        println!("Too bad! You drew. Try winning next time.");
    }
}
/// Function that runs if use decides to play with the engine
pub fn play_game_with_engine() {
    let mut board = board_new();
    let mut all_inputs: Vec<String> = Vec::new();
    let if_random = if_random();
    let user_player = choose_player();
    // Engine is opposite player type of player
    let engine_player = switch_player_turn(&user_player);
    println!("To choose a square pick a number between 1 and 9");
    loop {
        // Loop breaks if all inputs is = 9
        if all_inputs.len() == 9 {
            break;
        }
        // Prints an empty board when user is Xs
        if &user_player == &Player::PlayerOne && all_inputs.len() == 0{
            print_board(&board);
        }
        // This is here as the engine pushes it's number to all_input before
        // The user can put input thiers
        if all_inputs.len() == 8 && user_player == Player::PlayerOne {
            break;
        }
        if &user_player == &Player::PlayerOne {
            let mut player_input = receive_input();
            // If input exists call again
            while if_input_exists(&all_inputs, player_input.clone()) {
                player_input = receive_input();
            }
            // Update the board
            update_board_state(&mut board, &user_player, &player_input);
            // Push to collection
            all_inputs.push(player_input);
            // If win break the loop
            if did_win(&board, &user_player) {
                break;
            }
          // Else the engine is X which means it goes first
        } else {
            // Call run_engine
            let result = run_engine(board, &engine_player, &engine_player, &user_player, if_random, &mut all_inputs);
            // Board becomes the output of run_engine
            board = result.0;
            // All_iputs becomes the output of run_engine
            all_inputs = result.1;
            if did_win(&board, &engine_player) {
            break;
        } 
        // If this is not here if engine did not win it would continue
        // Softlocking the program as you can't input anymore numbers
        if all_inputs.len() == 9 {
            break;
        }
        // The board gets printed on the engines turn
        // When user was Circles it would print after you went
        // Which meant you couldn't see what the engine did
        print_board(&board);
        }
        // The same as for Xs but now for Circles
        if &user_player == &Player::PlayerTwo {
            let mut player_input = receive_input();
            while if_input_exists(&all_inputs, player_input.clone()) {
                player_input = receive_input();
            }
            update_board_state(&mut board, &user_player, &player_input);
            all_inputs.push(player_input);
            if did_win(&board, &user_player) {
                break;
            }
        } else {
            let result = run_engine(board, &engine_player, &engine_player, &user_player, if_random, &mut all_inputs);
            board = result.0;
            all_inputs = result.1;
            if did_win(&board, &engine_player) {
                break;
            }
            // Same as with the first one
            print_board(&board);
        }
        
    }
    // Another round of input for the user
    if all_inputs.len() == 8 && did_win(&board, &user_player) == false {
        // The engine can win when all_inputs = 8
        // So this catches that
        if did_win(&board, &engine_player) {
            print_board(&board);
            return println!("You lost to a bot! You suck!");
        }
        let mut player_input = receive_input();
            while if_input_exists(&all_inputs, player_input.clone()) {
                player_input = receive_input();
            }
            update_board_state(&mut board, &user_player, &player_input);
            all_inputs.push(player_input);
    }
    // Printing the statement and ending the loop if they lose or draw
    print_board(&board);
    if did_win(&board, &user_player) {
        return println!("You won against a bot! Don't feel to proud of yourself!");
    }
    if did_win(&board, &engine_player) {
        return println!("You lost to a bot! You suck!");
    }
    println!("Too bad! You drew. Try winning next time.");
    }