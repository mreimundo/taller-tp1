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
    other_executions::{ExecutionStage, execute_instruction},
    stack::Stack,
};
use std::iter::Peekable;
use std::str::Chars;

/// Function to handle the process of DotQuote expression. This process is constructed by:
/// 1. Consuming the initial `.` and optional leading spaces
/// 2. Collecting all characters received by parameter until a closing `"` is found
/// 3. Adding the formatted token (e.g., `."message"`) to the tokens vector received by parameter.

pub fn tokenize_dot_quote(chars: &mut Peekable<Chars>, tokens: &mut Vec<String>) {
    chars.next();
    let mut dot_quote = String::new();

    while let Some(' ') = chars.peek() {
        chars.next();
    }

    for c in chars.by_ref() {
        if c == '"' {
            break;
        }
        dot_quote.push(c);
    }

    tokens.push(format!(".\"{}", dot_quote));
}

/// Function that pushes the current token to the tokens if it is not empty.

pub fn handle_token_char(cur_tok: String, tokens: &mut Vec<String>) {
    if !cur_tok.is_empty() {
        tokens.push(cur_tok);
    }
}

/// Function that 'tokenize' the input received as &str, returning a vector of String.
/// This function iterates the characters, processing each one to return its interpretation in the following way:
/// 1. Splitting on whitespace (spaces and tabs)
/// 2. Handling special dot-quote strings (`."...`) as single tokens
/// 3. Preserving all other character sequences as distinct tokens

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

/// Function that parse a token.
/// Receives a token as &str and &WordsDictionary, returning its associated ForthValue.
/// This function attempts to interpret a token in the following priority order:
/// 1. Dot-quote strings (e.g., `."message"`)
/// 2. User-defined words (checks dictionary)
/// 3. Numeric literals
/// 4. Built-in operations (arithmetic, stack, output, boolean, conditional)
/// 5. Word definitions (start/end markers)

pub fn parse_token(token: &str, dictionary: &WordsDictionary) -> ForthValue {
    if let Some(quoted_text) = token.strip_prefix(".\"") {
        return ForthValue::Operation(ForthOperation::Output(OutputOperation::DotQuote(
            quoted_text.to_string(),
        )));
    }

    let uppercased_token = token.to_uppercase();

    if dictionary.word_already_defined(&uppercased_token) {
        return ForthValue::Word(ForthWord::Start(uppercased_token));
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
        Err(_) => ForthValue::Word(ForthWord::Start(uppercased_token)),
    }
}

/// Function used to handle values that are not a word.
/// Receives the ForthValue, a flag that indicates if a word is or is not defined, a mutable ForthValue vector "definition" to add a value if its defining a word,
/// and a mutable Stack, WordsDictionary, the executed words (vector of String), and the execution stage vector to pass directly to execute_instruction function.

pub fn handle_other_token(
    value: ForthValue,
    flag_defining_word: bool,
    definition: &mut Vec<ForthValue>,
    stack: &mut Stack,
    dictionary: &mut WordsDictionary,
    executed_words: &mut Vec<String>,
    execution_stage_stack: &mut Vec<ExecutionStage>,
) {
    if flag_defining_word {
        definition.push(value);
    } else {
        execute_instruction(
            &value,
            stack,
            dictionary,
            execution_stage_stack,
            None,
            executed_words,
        );
    }
}

/// Function used to process a sequence of tokens received by parameter as a reference list to String values.
/// It also receives a mutable Stack and WordsDictionary to change the values if necessary by passing to other functions.
/// This function is the core interpreter that:
/// 1. Manages word definition mode (between `:` and `;`)
/// 2. Handles execution flow control (if/else/then)
/// 3. Processes all other operations and literals
/// NOTE: This function seems too long, but it exceeds 30 lines of body by the way cargo fmt puts line breaks into invoked functions parameters. If we change the parameters to be inline, this would not happen.

pub fn read_tokens(tokens: &[String], stack: &mut Stack, dictionary: &mut WordsDictionary) {
    let mut i = 0;
    let mut flag_defining_word = false;
    let mut current_word_name = "";
    let mut current_definition = Vec::new();
    let mut executed_words = Vec::new();
    let mut execution_stage_stack = vec![ExecutionStage::Executing];
    while i < tokens.len() {
        let value = parse_token(&tokens[i], dictionary);
        match &value {
            ForthValue::Word(ForthWord::Definition) => {
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
            ForthValue::Word(ForthWord::End) => {
                handle_word_end(
                    &mut flag_defining_word,
                    current_word_name,
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
                    &mut execution_stage_stack,
                );
            }
        }
        i += 1;
    }
    if flag_defining_word {
        print_error(ForthError::InvalidWord);
    }
}
