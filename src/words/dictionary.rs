use crate::forth_value::ForthValue;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WordsDictionary {
    words: HashMap<String, Vec<ForthValue>>,
}

impl WordsDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, name: &str, definition: Vec<ForthValue>) {
        self.words.insert(name.to_uppercase(), definition);
    }

    pub fn get_word(&self, name: &str) -> Option<&Vec<ForthValue>> {
        self.words.get(&name.to_uppercase())
    }

    pub fn get_word_mut(&mut self, name: &str) -> Option<&mut Vec<ForthValue>> {
        self.words.get_mut(&name.to_uppercase())
    }

    pub fn word_already_defined(&self, name: &str) -> bool {
        self.words.contains_key(&name.to_uppercase())
    }
}
