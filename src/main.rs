use std::{io, process::exit};

fn main() {
    display_menu();

    let mut option;

    loop {
        // get user input
        option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        menu_selection(option);
    }
}

// Displays the menu options a user can select from
fn display_menu() {
    println!(
        "Welcome to Tic-Tac-Toe!
Please choose from the following options:

1 - One Player
2 - Two Player
3 - Quit"
    );
}

/// Determines what functions to call based on the menu selection of the user
fn menu_selection(option: u8) {
    match option {
        1 => start_one_player_game(),
        2 => start_two_player_game(),
        3 => exit_program(),
        _ => println!("Not a valid selection"),
    }
}

/// Initiates a one-player game
fn start_one_player_game() {
    println!("one player!")
}

/// Initiates a two-player game
fn start_two_player_game() {
    println!("two player!")
}

/// Exits the program
fn exit_program() {
    println!();
    println!("Thank you for playing!");
    println!("Shutting down...");
    exit(0)
}
