use std::collections::HashMap;

use bevy::{ prelude::{Handle, Image}};





pub struct Sprites {
    map: HashMap<String, Handle<Image>>,
}

#[allow(dead_code)]
impl Sprites {

    pub fn insert(&mut self, file_name: String, texture: Handle<Image>) {
        self.map.insert(file_name, texture);
    }

    pub fn get(&self, file_name: String) -> Option<&Handle<Image>> {
        self.map.get(&file_name)
    }

    pub fn get_map(self) -> HashMap<String, Handle<Image>>{
        self.map
    }
    pub fn default() -> Sprites {
        Sprites {
            map: HashMap::new()
        }
    }
}
