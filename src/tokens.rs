use crate::operations::{
    arithmetic::parse_arithmetic, boolean::parse_boolean, conditional::parse_conditional,
    output::parse_output, stack_type::parse_stack_op,
};
use crate::operations::{forth_operation::ForthOperation, output::OutputOperation};
use crate::words::{
    dictionary::WordsDictionary,
    word::{ForthWord, handle_word_definition, handle_word_end, parse_word},
};
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
};
use crate::{
    executions::{ExecutionStage, execute_instruction},
    stack::Stack,
};
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize_dot_quote(chars: &mut Peekable<Chars>, tokens: &mut Vec<String>) {
    chars.next();
    let mut dot_quote = String::new();

    while let Some(' ') = chars.peek() {
        chars.next();
    }

    while let Some(c) = chars.next() {
        if c == '"' {
            break;
        }
        dot_quote.push(c);
    }

    tokens.push(format!(".\"{}", dot_quote));
}

pub fn handle_token_char(cur_tok: String, tokens: &mut Vec<String>) {
    if !cur_tok.is_empty() {
        tokens.push(cur_tok);
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_token = String::new();

    while let Some(c) = chars.next() {
        match c {
            '.' if chars.peek() == Some(&'"') => {
                handle_token_char(current_token, &mut tokens);
                current_token = String::new();
                tokenize_dot_quote(&mut chars, &mut tokens);
            }
            ' ' | '\t' => {
                handle_token_char(current_token, &mut tokens);
                current_token = String::new();
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    handle_token_char(current_token, &mut tokens);

    tokens
}

/*recibe un token y devuelve la operación asociada al mismo.
si no encuentra una de las instrucciones básicas de forth, se trata de una word o de un número
*/
pub fn parse_token(token: &str, dictionary: &WordsDictionary) -> ForthValue {
    if token.starts_with(".\"") {
        let quoted_text = &token[2..];
        return ForthValue::Operation(ForthOperation::Output(OutputOperation::DotQuote(
            quoted_text.to_string(),
        )));
    }

    let uppercased_token = token.to_uppercase();

    if dictionary.word_already_defined(&uppercased_token) {
        return ForthValue::Word(ForthWord::WordStart(uppercased_token));
    }

    if let Some(value) = parse_arithmetic(&uppercased_token) {
        return value;
    }
    if let Some(value) = parse_stack_op(&uppercased_token) {
        return value;
    }
    if let Some(value) = parse_output(&uppercased_token) {
        return value;
    }
    if let Some(value) = parse_boolean(&uppercased_token) {
        return value;
    }
    if let Some(value) = parse_conditional(&uppercased_token) {
        return value;
    }
    if let Some(value) = parse_word(&uppercased_token) {
        return value;
    }

    match token.parse::<i16>() {
        Ok(num) => ForthValue::Number(num),
        Err(_) => ForthValue::Word(ForthWord::WordStart(uppercased_token)),
    }
}

pub fn handle_other_token(
    value: ForthValue,
    flag_defining_word: bool,
    definition: &mut Vec<ForthValue>,
    stack: &mut Stack,
    dictionary: &mut WordsDictionary,
    executed_words: &mut Vec<String>,
    execution_mode_stack: &mut Vec<ExecutionStage>,
) {
    if flag_defining_word {
        definition.push(value);
    } else {
        execute_instruction(
            &value,
            stack,
            dictionary,
            execution_mode_stack,
            None,
            executed_words,
        );
    }
}

/*función para leer los tokens.
realizando pattern matching se verifica si es una word, cuyo comportamiento se define aparte de las operaciones convencionales.
si no es de ese tipo o bien puede ejecutarse o está en medio de la definición de la word, por lo que se agrega el flag_defining_word
*/
pub fn read_tokens(tokens: &[String], stack: &mut Stack, dictionary: &mut WordsDictionary) {
    let mut i = 0;
    let mut flag_defining_word = false;
    let mut current_word_name = "";
    let mut current_definition = Vec::new();
    let mut executed_words = Vec::new();
    let mut execution_mode_stack = vec![ExecutionStage::Executing];

    while i < tokens.len() {
        let value = parse_token(&tokens[i], &dictionary);
        match &value {
            ForthValue::Word(ForthWord::WordDefinition) => {
                handle_word_definition(
                    tokens,
                    &mut i,
                    &mut flag_defining_word,
                    &mut current_word_name,
                    &mut current_definition,
                );
                if !flag_defining_word {
                    return;
                }
            }
            ForthValue::Word(ForthWord::WordEnd) => {
                handle_word_end(
                    &mut flag_defining_word,
                    &current_word_name,
                    &mut current_definition,
                    dictionary,
                );
                if flag_defining_word {
                    flag_defining_word = false;
                } else {
                    return;
                }
            }
            _ => {
                handle_other_token(
                    value,
                    flag_defining_word,
                    &mut current_definition,
                    stack,
                    dictionary,
                    &mut executed_words,
                    &mut execution_mode_stack,
                );
            }
        }

        i += 1;
    }

    if flag_defining_word {
        print_error(ForthError::InvalidWord);
    }
}
