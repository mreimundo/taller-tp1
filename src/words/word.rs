use super::dictionary::WordsDictionary;
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    utils::get_copy_forth_value,
};

#[derive(Debug)]
pub enum ForthWord {
    WordStart(String),
    WordDefinition,
    WordEnd,
}

pub fn handle_word_definition<'a>(
    tokens: &'a [String],
    i: &mut usize,
    flag: &mut bool,
    name: &mut &'a str,
    definition: &mut Vec<ForthValue>,
) {
    if *flag {
        print_error(ForthError::InvalidWord);
        return;
    }

    if *i + 1 >= tokens.len() {
        print_error(ForthError::InvalidWord);
        return;
    }

    let word_name = &tokens[*i + 1];

    if word_name.parse::<i16>().is_ok() {
        print_error(ForthError::InvalidWord);
        return;
    }

    *flag = true;
    *name = word_name;
    definition.clear();
    *i += 1;
}

pub fn handle_word_end(
    flag: &mut bool,
    name: &str,
    definition: &mut Vec<ForthValue>,
    dictionary: &mut WordsDictionary,
) {
    if *flag {
        let mut new_definition = Vec::with_capacity(definition.len());
        while let Some(item) = definition.pop() {
            new_definition.insert(0, item);
        }
        let mut final_definition = Vec::with_capacity(new_definition.len());
        for val in new_definition {
            if let ForthValue::Word(ForthWord::WordStart(ref word_name)) = val {
                if let Some(referenced_definition) = dictionary.get_word(word_name) {
                    for word_val in referenced_definition {
                        final_definition.push(get_copy_forth_value(word_val));
                    }
                    continue;
                }
            }
            final_definition.push(val);
        }
        if dictionary.word_already_defined(name) {
            if let Some(existing_definition) = dictionary.get_word_mut(name) {
                *existing_definition = final_definition;
            }
        } else {
            dictionary.add_word(name, final_definition);
        }
        *flag = false;
    } else {
        print_error(ForthError::InvalidWord);
    }
}

pub fn parse_word(token: &str) -> Option<ForthValue> {
    match token {
        ":" => Some(ForthValue::Word(ForthWord::WordDefinition)),
        ";" => Some(ForthValue::Word(ForthWord::WordEnd)),
        _ => None,
    }
}
