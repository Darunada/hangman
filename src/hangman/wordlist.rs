use std::{collections::hash_map::DefaultHasher, fs::File, hash::{Hash, Hasher}, io::{self, BufRead, BufReader, Lines}};
use rand::{prelude::*};

#[derive(Debug, Default)]
pub struct WordList {
    words: Vec<String>,
    hash: u64
}

impl WordList {
    pub fn from_path(path: &str) -> Result<WordList, std::io::Error> {
        let file = File::open(path).unwrap();
        let word_list: WordList = io::BufReader::new(file).lines().into();
        Ok(word_list)
    }

    fn new(lines: Lines<BufReader<File>>) -> WordList {
        let mut wordlist = WordList::default();

        for line in lines {
            match line {
                Ok(word) => wordlist.words.push(word.trim().to_string()),
                Err(_) => println!("Failed to read word."),
            }
        }
        let mut hasher = DefaultHasher::new();
        wordlist.words.hash(&mut hasher);
        wordlist.hash = hasher.finish();
        
        wordlist
    }

    pub fn get_random(&self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.words.len());
        self.words.get(index).unwrap().to_string()
    }
}

impl From<Lines<BufReader<File>>> for WordList {
    fn from(lines: Lines<BufReader<File>>) -> Self {
        WordList::new(lines)
    }
}

#[cfg(test)]
mod tests {

    use super::WordList;
    
    #[test]
    fn loads_words_into_wordlist() {
        let wordlist = WordList::from_path("test.words.txt").unwrap();
        assert_eq!(wordlist.words.len(), 3);
    }

    #[test]
    fn gets_a_random_word() {
        let wordlist = WordList::from_path("test.words.txt").unwrap();
        let valid_words = vec!["one", "two", "three"];
        
        let random = wordlist.get_random();
        assert!(valid_words.contains(&random.as_str()));
    }
}