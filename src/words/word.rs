use super::dictionary::WordsDictionary;
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    other_executions::{ExecutionStage, execute_instruction},
    stack::Stack,
    utils::get_copy_forth_value,
};

#[derive(Debug)]
pub enum ForthWord {
    Start(String),
    Definition,
    End,
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
            if let ForthValue::Word(ForthWord::Start(ref word_name)) = val {
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
        ":" => Some(ForthValue::Word(ForthWord::Definition)),
        ";" => Some(ForthValue::Word(ForthWord::End)),
        _ => None,
    }
}

/// Execute the word if valid. It is also pushed to a vector of String.
/// The function can execute other words contained in another one, allowing recursion and also redefinition.

pub fn handle_word_execution(
    word_name: &String,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    executed_words: &mut Vec<String>,
) {
    if executed_words.contains(word_name) {
        return;
    }

    executed_words.push(word_name.to_string());

    match dictionary.get_word(word_name) {
        Some(definition) => {
            let mut mode_stack = vec![ExecutionStage::Executing];
            for val in definition {
                execute_instruction(
                    val,
                    stack,
                    dictionary,
                    &mut mode_stack,
                    Some(word_name.to_string()),
                    executed_words,
                );
            }
        }
        None => print_error(ForthError::UnknownWord),
    }

    executed_words.pop();
}

#[cfg(test)]
mod tests {
    use crate::{
        stack::Stack,
        tokens::{read_tokens, tokenize},
        words::dictionary::WordsDictionary,
    };

    #[test]
    fn test_case_insensitive() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": foo dup ;"), &mut test_stack, &mut dict);

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("FOO Foo foo"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 1, 1, 1]);
    }

    #[test]
    fn test_word_definition() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(
            &tokenize(": dup-twice dup dup ;"),
            &mut test_stack,
            &mut dict,
        );

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("dup-twice"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 1, 1]);
    }

    #[test]
    fn test_redefinition() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": foo dup ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize(": foo dup dup ;"), &mut test_stack, &mut dict);

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("foo"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 1, 1]);
    }

    #[test]
    fn test_shadowing_builtin() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(
            &tokenize(": swap dup dup dup ;"),
            &mut test_stack,
            &mut dict,
        );

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("swap"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 1, 1, 1]);
    }

    #[test]
    fn test_non_transitive() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": foo 5 ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize(": bar foo ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize(": foo 6 ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize("bar foo"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[5, 6]);
    }

    #[test]
    fn test_self_referential() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": foo 10 ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize(": foo foo 1 + ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize("foo"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[11]);
    }

    #[test]
    fn test_shadowing_symbol() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": + * ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize("3 4 +"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[12]);
    }

    #[test]
    fn test_countup() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": countup 1 2 3 ;"), &mut test_stack, &mut dict);

        read_tokens(&tokenize("countup"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 2, 3]);
    }

    #[test]
    fn test_shadowing_dup() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": swap dup ;"), &mut test_stack, &mut dict);

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("swap"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 1]);
    }
}
