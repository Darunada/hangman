use hangman::{GameState, init, print_welcome, tick};
use std::io::{self, Read};
mod hangman;

fn main() {
    

    print_welcome();
    start_game();
    
}

fn start_game() {
    let mut hangman = init();
    loop {
        let state = tick(&hangman);
        match state {
            GameState::Running => {
                loop {
                    
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    let length = buffer.trim().len();
                    println!("");
                
                    if length == 1 {

                        let ch = buffer.to_ascii_lowercase().chars().next().unwrap();

                        match hangman.guess(&ch) {
                            Ok(Some(c)) => println!("You have matched {}.", c),
                            Ok(None) => println!("No Match"),
                            Err(hangman::BadGuess::Duplicate) => println!("You have already guessed {}.", buffer),
                            Err(hangman::BadGuess::Invalid) => println!("You have entered an invalid guess.  Select a letter from a-z only."),
                        };
                        break;
                    } else {
                        println!("Please enter exactly one character to guess: ");
                    }
                }
                
                
            },
            GameState::Victory => break,
            GameState::GameOver => break,

        }

    }
}

