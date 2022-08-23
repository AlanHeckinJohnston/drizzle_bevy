use std::fs;

use bevy::prelude::Commands;
use bevy::prelude::Res;
use property_parser::PropertyParser;
use queue::Queue;

use std::collections::{HashSet, HashMap};

use rand::thread_rng;
use rand::Rng;

use crate::resources::dict::Dict;




pub fn init_dictionary(mut commands: Commands, props: Res<PropertyParser>) {
    let contents = fs::read_to_string("resources/words_alpha.txt").unwrap();
    let word_split = contents.split("\r\n");

    let mut words: HashSet<String> = HashSet::new();

    for i in word_split {

        if i.to_string().len() < 3 {
            continue;
        }
        words.insert(i.to_string());
    }

    let mut letter_proportions: HashMap<char, i32> = HashMap::new();

    let mut i: u8 = 97; // lowercase a

    while i < 122 {
        letter_proportions.insert(i as char, props.get_property(String::from(i as char)).unwrap().get_int_value());
        i += 1;
    }

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

