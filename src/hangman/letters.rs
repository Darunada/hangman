use std::fmt::Display;

use super::{BadGuess, GuessResult};

#[derive(Debug)]
struct Letter {
    letter: char,
    guessed: bool,
}

impl Letter {
    fn guess(&mut self, guess: &char) -> GuessResult {
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


#[cfg(test)]
mod tests {

    use crate::hangman::BadGuess;

    use super::{Letter, Letters};

    #[test]
    fn guessing_a_letter_correct() {
        let mut letter = Letter {
            letter: 'a',
            guessed: false,
        };

        let result = letter.guess(&'a');
        assert_eq!(result.unwrap(), Some('a'));
    }

    #[test]
    fn guessing_a_letter_duplicate() {
        let mut letter = Letter {
            letter: 'a',
            guessed: true,
        };

        let result = letter.guess(&'a');
        assert_eq!(result.unwrap_err(), BadGuess::Duplicate);
    }

    #[test]
    fn guessing_a_letter_invalid() {
        let mut letter = Letter {
            letter: 'a',
            guessed: false,
        };

        // letter does not check for validity
        let result = letter.guess(&'!');
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn letters_contains_all_the_letters() {
        let word = "alphabetical";

        let mut letters = Letters::new(word).letters;
        
        word.chars().rev().for_each(|l| {
            assert_eq!(letters.last().unwrap().letter, l);
            letters.pop();
        });

        assert!(letters.is_empty());
    }

    #[test]
    fn guesses_match_one_letter() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);
        assert_eq!(letters.guess(&'p').unwrap(), Some('p'));

        let result = letters.letters;
        assert!(result[2].guessed);

        let mut unguessed_letters = 0;
        for letter in &result {
            if !letter.guessed {
                unguessed_letters += 1
            }
        };

        assert_eq!(unguessed_letters, 11);
    }

    
    #[test]
    fn guesses_match_many_letters() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);
        assert_eq!(letters.guess(&'a').unwrap(), Some('a'));

        let result = letters.letters;
        assert!(result[0].guessed);
        assert!(result[4].guessed);
        assert!(result[10].guessed);

        let mut unguessed_letters = 0;
        for letter in &result {
            if !letter.guessed {
                unguessed_letters += 1
            }
        };

        assert_eq!(unguessed_letters, 9);
    }

    #[test]
    fn guessing_a_word_correct() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);

        let result = letters.guess(&'a');
        assert_eq!(result.unwrap(), Some('a'));
    }

    #[test]
    fn guessing_a_word_none() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);

        let result = letters.guess(&'z');
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn guessing_a_word_duplicate() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);

        letters.guess(&'a').unwrap();
        let result = letters.guess(&'a');
        assert_eq!(result.unwrap_err(), BadGuess::Duplicate);
    }

    #[test]
    fn guessing_a_word_invalid() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);

        // letters does not check for validity
        let result = letters.guess(&'!');
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn guessing_a_whole_word_is_detected() {
        let word = "alphabetical";
        let mut letters = Letters::new(word);

        assert!(!letters.is_guessed());
        letters.guess(&'a').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'l').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'p').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'h').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'b').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'e').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'t').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'i').unwrap();
        assert!(!letters.is_guessed());
        letters.guess(&'c').unwrap();
        
        assert!(letters.is_guessed());
    }
}
