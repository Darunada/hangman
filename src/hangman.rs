

use crate::hangman::wordlist::WordList;
use crate::hangman::body::Body;
use crate::hangman::letters::Letters;
mod wordlist;
mod body;
mod letters;

#[derive(Debug, PartialEq)]
pub enum BadGuess {
    Duplicate,
    Invalid,
}

type GuessResult = Result<Option<char>, BadGuess>;
type Guesses = Vec<char>;

#[derive(Debug)]
pub enum GameState {
    Running,
    Victory,
    GameOver
}

#[derive(Debug)]
pub struct Hangman {
    letters: Letters,
    guesses: Guesses,
    body: Body
}

impl Hangman {
    fn new(word: &str) -> Self {
        Hangman { letters: Letters::new(word), guesses: vec![], body: Body::new() }
    }

    fn guesses_remaining(&self) -> Option<usize> {
        if self.guesses.len() <= 5 {
            Some(5 - self.guesses.len())
        } else {
            None
        }
    }

    pub fn guess(&mut self, guess: &char) -> GuessResult {
        if *guess < 'a' || *guess > 'z' {
            return Err(BadGuess::Invalid);
        }

        if self.guesses.contains(guess) {
            return Err(BadGuess::Duplicate);
        }

        let result = self.letters.guess(guess);
        if let Ok(None) = result {
            self.guesses.push(*guess);
            self.body.reveal();
        }

        result
    } 

    fn is_guessed(&self) -> bool {
        self.letters.is_guessed()
    }
}

pub fn init() -> Hangman {
    let word_list = WordList::from_path("words.txt").unwrap();
    let word: String = word_list.get_random();

    Hangman::new(word.as_str())
}

pub fn tick(hangman: &Hangman) -> GameState {
    println!("{}", hangman.body);
    println!();
    println!("{}", hangman.letters);
    println!();

    if hangman.is_guessed() {
        println!("Congratulations!  You have won.");
        GameState::Victory
    } else if let Some(guesses_remaining) = hangman.guesses_remaining() {
        println!("You have {} guesses remaining.", guesses_remaining);
        println!("Enter your guess: ");
        GameState::Running
    } else {
        println!("You have no guesses remaining!  You have lost.");
        GameState::GameOver
    }
}

pub fn print_welcome() {
    println!("
     _                                             
    | |                                            
    | |__   __ _ _ __   __ _ _ __ ___   __ _ _ __  
    | '_ \\ / _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\ 
    | | | | (_| | | | | (_| | | | | | | (_| | | | |
    |_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|
                        __/ |                      
                       |___/")
}
