mod guessing_game;

use guessing_game::guessing_game;
use std::io;

fn main() {
    loop {
        print_menu();

        println!("Enter your choice:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!\n");
                continue;
            }
        };

        match choice {
            0 => break,
            1 => guessing_game(),
            _ => println!("Invalid choice!\n"),
        }
    }
}

fn print_menu() {
    println!("Which program would you like to run? (Enter number)");
    println!("0. Exit");
    println!("1. Guessing Game\n");
}
