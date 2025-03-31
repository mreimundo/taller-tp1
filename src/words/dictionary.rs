use crate::forth_value::ForthValue;
use std::collections::HashMap;

/// This struct is used to handle the words defined in the program.
/// It contains words as field, which is a HashMap where the key is a String (word-name) and the value (word-body) is represented by a vector of ForthValue
///
#[derive(Debug, Default)]
pub struct WordsDictionary {
    words: HashMap<String, Vec<ForthValue>>,
}

impl WordsDictionary {
    /// Function used to build the structure. Initializes the WordsDictionary by default (mainly to fix a warning).
    pub fn new() -> Self {
        Self::default()
    }

    /// Function used to add a word to the dictionary. It receives a mutable instance of this, the name (key) of the word as &str, and a definition that is, as said, a vector of ForthValue.
    /// Inserts the key as uppercase to be insensitive case
    pub fn add_word(&mut self, name: &str, definition: Vec<ForthValue>) {
        self.words.insert(name.to_uppercase(), definition);
    }

    /// Function used to get a word of the current dictionary. It receives an instance of this and the name (key) of the word as &str.
    /// Gets the key as a reference to the current value (vector of ForthValue).
    pub fn get_word(&self, name: &str) -> Option<&Vec<ForthValue>> {
        self.words.get(name)
    }

    /// Function used to get a mutable word instance of the dictionary. It receives a mutable instance of this and the name (key) of the word as &str.
    /// Gets the key as a mutable reference to the current value (vector of ForthValue).
    pub fn get_word_mut(&mut self, name: &str) -> Option<&mut Vec<ForthValue>> {
        self.words.get_mut(name)
    }

    /// Function used to know if a word is or is not defined. It receives a reference to the dictionary and the name (key) of the word as &str.
    /// Returns true if it is in the dictionary instance, or false if it is not.
    pub fn word_already_defined(&self, name: &str) -> bool {
        self.words.contains_key(name)
    }
}
