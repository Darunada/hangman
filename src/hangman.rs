
use std::fmt::Display;

use crate::hangman::wordlist::WordList;
use crate::hangman::body::Body;

mod wordlist;
mod body;

#[derive(Debug)]
pub enum BadGuess {
    Duplicate,
    Invalid,
}

type GuessResult = Result<Option<char>, BadGuess>;

#[derive(Debug)]
struct Letter {
    letter: char,
    guessed: bool,
}

impl Letter {
    fn guess(&mut self, guess: &char) -> GuessResult {
        if self.letter.eq(guess) && self.guessed == true {
            Err(BadGuess::Duplicate)
        } else if self.letter.eq(guess) {
            self.guessed = true;
            Ok(Some(guess.clone()))
        } else {
            Ok(None)
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.guessed {
            write!(f, "{}", self.letter)
        } else {
            write!(f, "{}", '_')
        }
    }
}

#[derive(Debug, Default)]
struct Letters {
    letters: Vec<Letter>,
}

impl Letters {
    fn new(word: &str) -> Self {
        let mut letters: Vec<Letter> = vec![];
        for c in word.chars() {
            letters.push(Letter {
                letter: c,
                guessed: false,
            });
        }

        Letters { letters }
    }

    fn guess(&mut self, guess: &char) -> GuessResult {
        let mut guess_results: Vec<GuessResult> = vec![];
        for letter in self.letters.iter_mut() {
            guess_results.push(letter.guess(guess));
        }

        let mut thinned: Vec<GuessResult> = guess_results.into_iter().filter(|item| match item {
            Ok(None) => false,
            _ => true,
        }).collect();

        // should now contain 0..n Err(BadGuess::Duplicate) or Some(chars)
        if thinned.len() > 0 {
            thinned.pop().unwrap()
        } else {
            Ok(None)
        }
    }
}

impl Display for Letters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self.letters.iter() {
            write!(f, "{} ", letter)?;
        }
        write!(f, "")
    }
}

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
        match result {
            Ok(None) => {
                self.guesses.push(guess.clone());
                self.body.reveal();
            },
            _ => {},
        }

        return result;
    } 

    fn is_guessed(&self) -> bool {
        !self.letters.letters.iter().any(|letter| letter.guessed == false )
    }
}

pub fn init() -> Hangman {
    let word_list = WordList::from_path(&"words.txt").unwrap();
    let word: String = word_list.get_random();

    Hangman::new(word.as_str())
}

pub fn tick(hangman: &Hangman) -> GameState {
    println!("{}", hangman.body);
    println!("");
    println!("{}", hangman.letters);
    println!("");

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
