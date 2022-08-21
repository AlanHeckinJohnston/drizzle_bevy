
use std::collections::{HashSet, HashMap};
use queue::Queue;

use crate::systems::init_dictionary::get_fresh_deck;

pub struct Dict {
    words: HashSet<String>,
    deck: Queue<char>,
    letter_proportions: HashMap<char, i32>
}

impl Dict {

    pub fn new(deck: Queue<char>, words: HashSet<String>, letter_proportions: HashMap<char, i32>) -> Dict {
        Dict {
            deck,
            words,
            letter_proportions
        }
    }
    pub fn get_letter(&mut self) -> char{
        match self.deck.dequeue() {
            Option::Some(val) => return val,
            Option::None => {
                self.deck = get_fresh_deck(&self.letter_proportions);
                self.get_letter()
            }
        }
    }

    pub fn is_word(&self, word: String) -> bool {
        self.words.contains(&word)
    }
}