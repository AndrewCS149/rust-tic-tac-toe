use std::{io, process::exit};

fn main() {
    display_menu();

    let mut option;

    loop {
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
        1 => first_selection(),
        2 => second_selection(),
        3 => exit_program(),
        _ => println!("Not a valid selection"),
    }
}

/// Initiates a one-player game
fn first_selection() {
    println!("one player!")
}

/// Initiates a two-player game
fn second_selection() {
    println!("two player!")
}

/// Exits the program
fn exit_program() {
    println!("Thank you for playing!");
    println!("Shutting down...");
    exit(0)
}
