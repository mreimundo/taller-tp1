mod errors;

use errors::{ForthError, print_error};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

const DEFAULT_STACK_SIZE: usize = 1024 * 128; //128KB
const STACK_REST_PATHNAME: &'static str = "stack.fth";

/*-------------- TODO CHECKLIST --------------
- Falta validar stack overflow y casos de test_error
- Separación en archivos
- Tests: a la misma altura que src pero en módulos apartes tipo crate. Implementarlos usando #[cfg(test)] en c/u. No se testea main.rs
- cargo fmt y cargo clippy (fix warnings)
- completar cargo doc
- Probar compilación y ejecución en una distro de linux. Validar todos los casos posibles
- Reducir líneas de algunas funciones
- Corregir nested ifs
*/

#[derive(Debug)]
enum ForthValue {
    Operation(ForthOperation),
    Word(ForthWord),
    Number(i16),
}

#[derive(Debug)]
enum ForthOperation {
    Arithmetic(ArithmeticOperation),
    StackTypeOp(StackOperation),
    Output(OutputOperation),
    Boolean(BooleanOperation),
    Conditional(ConditionalOperation),
}

#[derive(Debug)]
enum ArithmeticOperation {
    Add,
    Substract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum StackOperation {
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate,
}

#[derive(Debug)]
enum ForthWord {
    WordStart(String),
    WordDefinition,
    WordEnd,
}

#[derive(Debug)]
enum OutputOperation {
    Dot,
    Emit,
    Cr,
    DotQuote(String),
}

#[derive(Debug)]
enum BooleanOperation {
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not,
}

#[derive(Debug)]
enum ConditionalOperation {
    If,
    Then,
    Else,
}
#[derive(PartialEq)]
enum ExecutionMode {
    Executing,
    Skipping(usize),
}

#[derive(Debug)]
struct Stack {
    data: Vec<i16>,
    max_elements: usize,
}

impl Stack {
    fn new(size: usize) -> Self {
        let max_elements = size / 2;
        Stack {
            data: Vec::with_capacity(max_elements),
            max_elements,
        }
    }

    fn push(&mut self, value: i16) -> Result<(), ForthError> {
        if self.data.len() >= self.max_elements {
            Err(ForthError::StackOverflow)
        } else {
            self.data.push(value);
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<i16, ForthError> {
        self.data.pop().ok_or(ForthError::StackUnderflow)
    }

    fn peek(&self) -> Result<&i16, ForthError> {
        self.data.last().ok_or(ForthError::StackUnderflow)
    }

    fn write_into_file(&mut self) -> io::Result<bool> {
        let stack_results: Vec<String> = self.data.iter().map(|&item| item.to_string()).collect();
        fs::write(STACK_REST_PATHNAME, stack_results.join(" "))?;
        Ok(true)
    }
}

#[derive(Debug)]
struct WordsDictionary {
    words: HashMap<String, Vec<ForthValue>>,
}

impl WordsDictionary {
    fn new() -> Self {
        WordsDictionary {
            words: HashMap::new(),
        }
    }

    fn add_word(&mut self, name: &str, definition: Vec<ForthValue>) {
        self.words.insert(name.to_uppercase(), definition);
    }

    fn get_word(&self, name: &str) -> Option<&Vec<ForthValue>> {
        self.words.get(&name.to_uppercase())
    }

    fn get_word_mut(&mut self, name: &str) -> Option<&mut Vec<ForthValue>> {
        self.words.get_mut(&name.to_uppercase())
    }

    fn word_already_defined(&self, name: &str) -> bool {
        self.words.contains_key(&name.to_uppercase())
    }
}

fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn tokenize_dot_quote(chars: &mut Peekable<Chars>, tokens: &mut Vec<String>) {
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

fn handle_token_char(cur_tok: String, tokens: &mut Vec<String>) {
    if !cur_tok.is_empty() {
        tokens.push(cur_tok);
    }
}

fn tokenize(input: &str) -> Vec<String> {
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

fn parse_arithmetic(token: &str) -> Option<ForthValue> {
    match token {
        "+" => Some(ForthValue::Operation(ForthOperation::Arithmetic(
            ArithmeticOperation::Add,
        ))),
        "-" => Some(ForthValue::Operation(ForthOperation::Arithmetic(
            ArithmeticOperation::Substract,
        ))),
        "*" => Some(ForthValue::Operation(ForthOperation::Arithmetic(
            ArithmeticOperation::Multiply,
        ))),
        "/" => Some(ForthValue::Operation(ForthOperation::Arithmetic(
            ArithmeticOperation::Divide,
        ))),
        _ => None,
    }
}

fn parse_stack_op(token: &str) -> Option<ForthValue> {
    match token {
        "DUP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Duplicate,
        ))),
        "DROP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Drop,
        ))),
        "SWAP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Swap,
        ))),
        "OVER" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Over,
        ))),
        "ROT" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Rotate,
        ))),
        _ => None,
    }
}

fn parse_output(token: &str) -> Option<ForthValue> {
    match token {
        "." => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Dot,
        ))),
        "EMIT" => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Emit,
        ))),
        "CR" => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Cr,
        ))),
        _ => None,
    }
}

fn parse_boolean(token: &str) -> Option<ForthValue> {
    match token {
        "=" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::Equal,
        ))),
        "<" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::Less,
        ))),
        ">" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::Greater,
        ))),
        "AND" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::And,
        ))),
        "OR" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::Or,
        ))),
        "NOT" => Some(ForthValue::Operation(ForthOperation::Boolean(
            BooleanOperation::Not,
        ))),
        _ => None,
    }
}

fn parse_conditional(token: &str) -> Option<ForthValue> {
    match token {
        "IF" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::If,
        ))),
        "THEN" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::Then,
        ))),
        "ELSE" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::Else,
        ))),
        _ => None,
    }
}

fn parse_word(token: &str) -> Option<ForthValue> {
    match token {
        ":" => Some(ForthValue::Word(ForthWord::WordDefinition)),
        ";" => Some(ForthValue::Word(ForthWord::WordEnd)),
        _ => None,
    }
}

/*recibe un token y devuelve la operación asociada al mismo.
si no encuentra una de las instrucciones básicas de forth, se trata de una word o de un número
*/
fn parse_token(token: &str, dictionary: &WordsDictionary) -> ForthValue {
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

fn execute_arithmetic_op(op: &ArithmeticOperation, stack: &mut Stack) {
    let a = match stack.pop() {
        Ok(val) => val,
        Err(e) => {
            print_error(e);
            return;
        }
    };
    let b = match stack.pop() {
        Ok(val) => val,
        Err(e) => {
            print_error(e);
            return;
        }
    };

    let result = match op {
        ArithmeticOperation::Add => a + b,
        ArithmeticOperation::Substract => b - a,
        ArithmeticOperation::Multiply => a * b,
        ArithmeticOperation::Divide => {
            if a != 0 {
                b / a
            } else {
                print_error(ForthError::DivisionByZero);
                return;
            }
        }
    };

    if let Err(e) = stack.push(result) {
        print_error(e);
    }
}

fn execute_stack_op(op: &StackOperation, stack: &mut Stack) {
    match op {
        StackOperation::Duplicate => match stack.peek() {
            Ok(a) => {
                if let Err(e) = stack.push(*a) {
                    print_error(e);
                }
            }
            Err(e) => print_error(e),
        },
        StackOperation::Drop => {
            if let Err(e) = stack.pop() {
                print_error(e);
            }
        }
        StackOperation::Swap => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Over => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Rotate => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let c = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(c) {
                print_error(e);
            }
        }
    }
}

fn execute_output_op(op: &OutputOperation, stack: &mut Stack) {
    match op {
        OutputOperation::Dot => match stack.pop() {
            Ok(a) => println!("{a}"),
            Err(e) => print_error(e),
        },
        OutputOperation::Cr => {
            println!();
        }
        OutputOperation::Emit => match stack.pop() {
            Ok(a) => {
                let ascii = a as u8;
                println!("{}", ascii as char);
            }
            Err(e) => print_error(e),
        },
        OutputOperation::DotQuote(text) => {
            println!("{text}");
        }
    }
}

fn execute_boolean_op(op: &BooleanOperation, stack: &mut Stack) {
    match op {
        BooleanOperation::Not => match stack.pop() {
            Ok(a) => {
                let result = if a != 0 { 0 } else { -1 };
                if let Err(e) = stack.push(result) {
                    print_error(e);
                }
            }
            Err(e) => print_error(e),
        },
        _ => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let result = match op {
                BooleanOperation::Equal => a == b,
                BooleanOperation::Greater => a < b,
                BooleanOperation::Less => a > b,
                BooleanOperation::And => a == -1 && b == -1,
                BooleanOperation::Or => a == -1 || b == -1,
                _ => {
                    print_error(ForthError::Generic("Unknown boolean operation"));
                    false
                }
            };
            if let Err(e) = stack.push(if result { -1 } else { 0 }) {
                print_error(e);
            }
        }
    }
}

fn execute_conditional_op(
    op: &ConditionalOperation,
    stack: &mut Stack,
    execution_mode: &mut Vec<ExecutionMode>,
) {
    match op {
        ConditionalOperation::If => match stack.pop() {
            Ok(condition) => {
                if condition == 0 {
                    execution_mode.push(ExecutionMode::Skipping(1));
                } else {
                    execution_mode.push(ExecutionMode::Executing);
                }
            }
            Err(e) => print_error(e),
        },
        ConditionalOperation::Else => {
            if let Some(last) = execution_mode.last_mut() {
                match last {
                    ExecutionMode::Executing => {
                        *last = ExecutionMode::Skipping(1);
                    }
                    ExecutionMode::Skipping(depth) => {
                        if *depth == 1 {
                            if let Some(mode) = execution_mode.last_mut() {
                                *mode = ExecutionMode::Executing;
                            }
                        }
                    }
                }
            }
        }
        ConditionalOperation::Then => {
            if let Some(last) = execution_mode.last_mut() {
                match last {
                    ExecutionMode::Skipping(depth) => {
                        if *depth > 1 {
                            *depth -= 1;
                        } else {
                            execution_mode.pop();
                        }
                    }
                    _ => {
                        execution_mode.pop();
                    }
                }
            }
        }
    }
}

fn execute_other_operations(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match val {
        ForthValue::Operation(ForthOperation::Arithmetic(op)) => execute_arithmetic_op(op, stack),
        ForthValue::Operation(ForthOperation::StackTypeOp(op)) => execute_stack_op(op, stack),
        ForthValue::Operation(ForthOperation::Output(op)) => execute_output_op(op, stack),
        ForthValue::Operation(ForthOperation::Boolean(op)) => execute_boolean_op(op, stack),
        ForthValue::Number(n) => {
            if let Err(e) = stack.push(*n) {
                print_error(e);
            }
        }
        ForthValue::Word(ForthWord::WordStart(word_name)) => {
            if let Some(ref current) = current_word {
                if current == word_name {
                    return;
                }
            }

            if let Some(definition) = dictionary.get_word(word_name) {
                let mut execution_mode_stack = vec![ExecutionMode::Executing];
                for val in definition {
                    execute_instruction(
                        val,
                        stack,
                        dictionary,
                        &mut execution_mode_stack,
                        Some(word_name.to_string()),
                        executed_words,
                    );
                }
            } else {
                print_error(ForthError::UnknownWord);
            }
        }
        _ => {}
    }
}

fn execute_instruction(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    execution_mode: &mut Vec<ExecutionMode>,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match execution_mode.last().unwrap_or(&ExecutionMode::Executing) {
        ExecutionMode::Executing => match val {
            ForthValue::Word(ForthWord::WordStart(word_name)) => {
                if executed_words.contains(word_name) {
                    return;
                }

                executed_words.push(word_name.to_string());

                if let Some(definition) = dictionary.get_word(word_name) {
                    let mut mode_stack = vec![ExecutionMode::Executing];
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
                } else {
                    print_error(ForthError::UnknownWord);
                }

                executed_words.pop();
            }
            ForthValue::Operation(ForthOperation::Conditional(op)) => {
                execute_conditional_op(op, stack, execution_mode);
            }
            _ => execute_other_operations(val, stack, dictionary, current_word, executed_words),
        },
        ExecutionMode::Skipping(_depth) => match val {
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Then)) => {
                if let Some(ExecutionMode::Skipping(current_depth)) = execution_mode.last_mut() {
                    if *current_depth > 1 {
                        *current_depth -= 1;
                    } else {
                        execution_mode.pop();
                    }
                }
            }
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Else)) => {
                if let Some(ExecutionMode::Skipping(depth)) = execution_mode.last_mut() {
                    if *depth == 1 {
                        if let Some(last) = execution_mode.last_mut() {
                            *last = ExecutionMode::Executing;
                        }
                    }
                }
            }
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::If)) => {
                if let Some(ExecutionMode::Skipping(depth)) = execution_mode.last_mut() {
                    *depth += 1;
                }
            }
            _ => {}
        },
    }
}

fn handle_word_definition<'a>(
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

fn get_copy_forth_value(value: &ForthValue) -> ForthValue {
    match value {
        ForthValue::Operation(op) => ForthValue::Operation(match op {
            ForthOperation::Arithmetic(a) => ForthOperation::Arithmetic(match a {
                ArithmeticOperation::Add => ArithmeticOperation::Add,
                ArithmeticOperation::Substract => ArithmeticOperation::Substract,
                ArithmeticOperation::Multiply => ArithmeticOperation::Multiply,
                ArithmeticOperation::Divide => ArithmeticOperation::Divide,
            }),
            ForthOperation::StackTypeOp(s) => ForthOperation::StackTypeOp(match s {
                StackOperation::Duplicate => StackOperation::Duplicate,
                StackOperation::Drop => StackOperation::Drop,
                StackOperation::Swap => StackOperation::Swap,
                StackOperation::Over => StackOperation::Over,
                StackOperation::Rotate => StackOperation::Rotate,
            }),
            ForthOperation::Output(o) => ForthOperation::Output(match o {
                OutputOperation::Dot => OutputOperation::Dot,
                OutputOperation::Emit => OutputOperation::Emit,
                OutputOperation::Cr => OutputOperation::Cr,
                OutputOperation::DotQuote(text) => OutputOperation::DotQuote(text.to_string()),
            }),
            ForthOperation::Boolean(b) => ForthOperation::Boolean(match b {
                BooleanOperation::Equal => BooleanOperation::Equal,
                BooleanOperation::Less => BooleanOperation::Less,
                BooleanOperation::Greater => BooleanOperation::Greater,
                BooleanOperation::And => BooleanOperation::And,
                BooleanOperation::Or => BooleanOperation::Or,
                BooleanOperation::Not => BooleanOperation::Not,
            }),
            ForthOperation::Conditional(c) => ForthOperation::Conditional(match c {
                ConditionalOperation::If => ConditionalOperation::If,
                ConditionalOperation::Then => ConditionalOperation::Then,
                ConditionalOperation::Else => ConditionalOperation::Else,
            }),
        }),
        ForthValue::Word(w) => ForthValue::Word(match w {
            ForthWord::WordStart(s) => ForthWord::WordStart(s.to_string()),
            ForthWord::WordDefinition => ForthWord::WordDefinition,
            ForthWord::WordEnd => ForthWord::WordEnd,
        }),
        ForthValue::Number(n) => ForthValue::Number(*n),
    }
}

fn handle_word_end(
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

fn handle_other_token(
    value: ForthValue,
    flag_defining_word: bool,
    definition: &mut Vec<ForthValue>,
    stack: &mut Stack,
    dictionary: &mut WordsDictionary,
    executed_words: &mut Vec<String>,
    execution_mode_stack: &mut Vec<ExecutionMode>,
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
fn read_tokens(tokens: &[String], stack: &mut Stack, dictionary: &mut WordsDictionary) {
    let mut i = 0;
    let mut flag_defining_word = false;
    let mut current_word_name = "";
    let mut current_definition = Vec::new();
    let mut executed_words = Vec::new();
    let mut execution_mode_stack = vec![ExecutionMode::Executing];

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

fn main() {
    println!("----- Intérprete básico de Forth -----");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_error(ForthError::WrongInput);
        return;
    }

    let size_bytes = args
        .get(2)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(DEFAULT_STACK_SIZE);

    let mut stack = Stack::new(size_bytes);
    let mut words_dictionary = WordsDictionary::new();

    if let Ok(lines) = read_file(&args[1]) {
        for line in lines {
            let tokens = tokenize(&line);
            read_tokens(&tokens, &mut stack, &mut words_dictionary);
        }

        match stack.write_into_file() {
            Ok(_) => println!(
                "Stack restante ({:?}) escrito en {}!",
                stack.data, STACK_REST_PATHNAME
            ),
            Err(_) => print_error(ForthError::Generic("Impossible to write stack")),
        }
    } else {
        print_error(ForthError::Generic("Impossible to read file.fth"));
    }
}
