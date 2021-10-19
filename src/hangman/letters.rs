use std::fmt::Display;

use super::{BadGuess, GuessResult};


#[derive(Debug)]
struct Letter {
    letter: char,
    guessed: bool,
}

impl Letter {
    pub fn guess(&mut self, guess: &char) -> GuessResult {
        if self.letter.eq(guess) && self.guessed {
            Err(BadGuess::Duplicate)
        } else if self.letter.eq(guess) {
            self.guessed = true;
            Ok(Some(*guess))
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
            write!(f, "_")
        }
    }
}

#[derive(Debug, Default)]
pub struct Letters {
    letters: Vec<Letter>,
}

impl Letters {
    pub fn new(word: &str) -> Self {
        let mut letters: Vec<Letter> = vec![];
        for c in word.chars() {
            letters.push(Letter {
                letter: c,
                guessed: false,
            });
        }

        Letters { letters }
    }

    pub fn guess(&mut self, guess: &char) -> GuessResult {
        let mut guess_results: Vec<GuessResult> = vec![];
        for letter in self.letters.iter_mut() {
            guess_results.push(letter.guess(guess));
        }

        let mut thinned: Vec<GuessResult> = guess_results.into_iter()
                                                         .filter(|item| !matches!(item, Ok(None)))
                                                         .collect();

        // should now contain 0..n Err(BadGuess::Duplicate) or Some(chars)
        if !thinned.is_empty() {
            thinned.pop().unwrap()
        } else {
            Ok(None)
        }
    }

    pub fn is_guessed(&self) -> bool {
        !self.letters.iter().any(|letter| !letter.guessed )
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