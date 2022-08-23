use std::fs;

use bevy::prelude::Commands;
use queue::Queue;

use std::collections::{HashSet, HashMap};

use rand::thread_rng;
use rand::Rng;

use crate::resources::dict::Dict;




pub fn init_dictionary(mut commands: Commands) {
    let contents = fs::read_to_string("resources/words_alpha.txt").unwrap();
    let word_split = contents.split("\r\n");

    let mut words: HashSet<String> = HashSet::new();
    let mut letter_usages: HashMap<char, i32> = HashMap::new();
    
    


    for i in word_split {

        if i.to_string().len() < 3 {
            continue;
        }
        words.insert(i.to_string());

        let letters_used = count_letters(i);

        for (character, used_in_word) in letters_used.to_owned() {
            
            let used = letter_usages.get(&character);

            if used.is_some() {
                letter_usages.insert(character, used.unwrap() + used_in_word);
            } else {
                letter_usages.insert(character, used_in_word);
            }
        }
    }

    let letter_proportions = get_proportion_of(letter_usages, 200);

    commands.insert_resource(Dict::new(
        get_fresh_deck(&letter_proportions),
        words,
        letter_proportions
    ));

}



fn flatten_deck(letter_proportions: &HashMap<char, i32>) -> Vec<char> {

    let mut result: Vec<char> = Vec::new();
    for (character, amount) in letter_proportions {
        let mut i = 0;

        while i < *amount {
            result.push(*character);
            i += 1;
        }
    }

    return result;
}


fn get_proportion_of(letter_use: HashMap<char, i32>, scale_to: i32) -> HashMap<char, i32>{
    
    let mut result: HashMap<char, i32> = HashMap::new();
    let total = get_total(&letter_use);

    for (k, v) in letter_use {
        
        result.insert(k, ((v as f64 / total as f64) * scale_to as f64) as i32);
    }

    result
}


fn get_total(map: &HashMap<char, i32>) -> i32 {
    
    let mut result = 0;
    for (_, v) in map {
        result += v;
    }

    result
}

fn count_letters(string: &str) -> HashMap<char, i32>{

    let mut result: HashMap<char, i32> = HashMap::new();
    for character in string.to_string().chars() {
        
        if !result.contains_key(&character) {
            result.insert(character, 1);
        }
        else {
            result.insert(character, result.get(&character).unwrap() + 1);
        }
    }

    result
}


pub fn get_fresh_deck(letter_proportions: &HashMap<char, i32>) -> Queue<char>{
    
    let mut queue: Queue<char> = Queue::new();
    let mut ordered_deck = flatten_deck(&letter_proportions);


    let mut rand = thread_rng();

    while ordered_deck.len() > 0 {
        let x = rand.gen_range(0..ordered_deck.len());
        
        let _ = queue.queue(ordered_deck.remove(x));

    }

    queue

}

