use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

const DEFAULT_STACK_SIZE: usize = 1024 * 128; //128KB
const DEFAULT_STACK_SIZE_AS_STR: &'static str = "131072"; //128KB en string


/*-------------- TODO CHECKLIST --------------
    - Escribir el stack restante luego de leer el path file (y ejecutar todo) en un nuevo archivo stack.fth. Formato: si el stack es [1, 2] escribirle 1 2
    - Manejo de errores: implementación y pensar si vale la pena usar structs o std:error / similares de std
    - Separación en archivos
    - Tests: a la misma altura que src pero en módulos apartes tipo crate. Implementarlos usando #[cfg(test)] en c/u. No se testea main.rs
    - cargo fmt y cargo clippy (fix warnings)
    - completar cargo doc
    - Probar compilación y ejecución en una distro de linux. Validar todos los casos posibles
*/

#[derive(Debug)]
enum ForthValue {
    Operation(ForthOperation),
    Word(ForthWord),
    Number(i16),
    Unknown(String)
}

#[derive(Debug)]
enum ForthOperation {
    Arithmetic(ArithmeticOperation),
    StackTypeOp(StackOperation),
    Output(OutputOperation),
    Boolean(BooleanOperation),
    Conditional(ConditionalOperation)
}

#[derive(Debug)]
enum ArithmeticOperation {
    Add,
    Substract,
    Multiply,
    Divide
}

#[derive(Debug)]
enum StackOperation {
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate
}

#[derive(Debug)]
enum ForthWord {
    WordStart(String),
    WordDefinition,
    WordEnd
}

#[derive(Debug)]
enum OutputOperation {
    Dot,
    Emit,
    Cr,
    DotQuote(String)
}

#[derive(Debug)]
enum BooleanOperation {
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not
}

#[derive(Debug)]
enum ConditionalOperation {
    If,
    Then,
    Else
}
#[derive(PartialEq)]
enum ExecutionMode { //se creó más que nada para manejar los stages en los if else then
    Executing,
    SkippingIf,
    SkippingElse
}


#[derive(Debug)]
struct Stack(Vec<i16>);

impl Stack {
    fn new(size: usize) -> Self {
        Stack(Vec::with_capacity(size))
    }
    
    fn push(&mut self, value: i16) {
        self.0.push(value);
    }
    
    fn pop(&mut self) -> Option<i16> {
        self.0.pop()
    }
    
    fn peek(&self) -> Option<&i16> {
        self.0.last()
    }
}

#[derive(Debug)]
struct WordsDictionary {
    words: HashMap<String, Vec<ForthValue>>
}

impl WordsDictionary {
    fn new() -> Self {
        WordsDictionary {
            words: HashMap::new()
        }
    }
    
    fn add_word(&mut self, name: &str, definition: Vec<ForthValue>) {
        self.words.insert(name.to_string(), definition);
    }
    
    fn get_word(&self, name: &str) -> Option<&Vec<ForthValue>> {
        self.words.get(name)
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
            _ => { current_token.push(c); }
        }
    }

    handle_token_char(current_token, &mut tokens);

    tokens
}


fn parse_arithmetic(token: &str) -> Option<ForthValue> {
    match token {
        "+" => Some(ForthValue::Operation(ForthOperation::Arithmetic(ArithmeticOperation::Add))),
        "-" => Some(ForthValue::Operation(ForthOperation::Arithmetic(ArithmeticOperation::Substract))),
        "*" => Some(ForthValue::Operation(ForthOperation::Arithmetic(ArithmeticOperation::Multiply))),
        "/" => Some(ForthValue::Operation(ForthOperation::Arithmetic(ArithmeticOperation::Divide))),
        _ => None,
    }
}

fn parse_stack_op(token: &str) -> Option<ForthValue> {
    match token {
        "DUP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(StackOperation::Duplicate))),
        "DROP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(StackOperation::Drop))),
        "SWAP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(StackOperation::Swap))),
        "OVER" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(StackOperation::Over))),
        "ROT" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(StackOperation::Rotate))),
        _ => None,
    }
}

fn parse_output(token: &str) -> Option<ForthValue> {
    match token {
        "." => Some(ForthValue::Operation(ForthOperation::Output(OutputOperation::Dot))),
        "EMIT" => Some(ForthValue::Operation(ForthOperation::Output(OutputOperation::Emit))),
        "CR" => Some(ForthValue::Operation(ForthOperation::Output(OutputOperation::Cr))),
        _ => None,
    }
}

fn parse_boolean(token: &str) -> Option<ForthValue> {
    match token {
        "=" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::Equal))),
        "<" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::Less))),
        ">" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::Greater))),
        "AND" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::And))),
        "OR" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::Or))),
        "INVERT" => Some(ForthValue::Operation(ForthOperation::Boolean(BooleanOperation::Not))),
        _ => None,
    }
}

fn parse_conditional(token: &str) -> Option<ForthValue> {
    match token {
        "IF" => Some(ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::If))),
        "THEN" => Some(ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Then))),
        "ELSE" => Some(ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Else))),
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
fn parse_token(token: &str) -> ForthValue {
    if token.starts_with(".\"") {
        let quoted_text = &token[2..];
        return ForthValue::Operation(ForthOperation::Output(OutputOperation::DotQuote(quoted_text.to_string())));
    }

    let uppercased_token = token.to_uppercase();
    let valid_token = uppercased_token.as_str();
    //se hizo asi para segmentarlo mejor y no tener un match gigante
    if let Some(value) = parse_arithmetic(valid_token) { return value; }
    if let Some(value) = parse_stack_op(valid_token) { return value; }
    if let Some(value) = parse_output(valid_token) { return value; }
    if let Some(value) = parse_boolean(valid_token) { return value; }
    if let Some(value) = parse_conditional(valid_token) { return value; }
    if let Some(value) = parse_word(valid_token) { return value; }

    match token.parse::<i16>() {
        Ok(num) => ForthValue::Number(num),
        Err(_) => ForthValue::Word(ForthWord::WordStart(token.to_string())),
    }
}


fn execute_arithmetic_op(op: &ArithmeticOperation, stack: &mut Stack) {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        let result = match op {
            ArithmeticOperation::Add => a + b,
            ArithmeticOperation::Substract => b - a,
            ArithmeticOperation::Multiply => a * b,
            ArithmeticOperation::Divide => b / a, // Asume que a ≠ 0 (manejo de error pendiente)
        };
        println!("Resultado operación: {}", result); //borrar dsp
        stack.push(result);
    }
}

fn execute_stack_op(op: &StackOperation, stack: &mut Stack) {
    match op {
        StackOperation::Duplicate => {
            if let Some(a) = stack.peek() {
                stack.push(*a);
            }
        }
        StackOperation::Drop => {
            if let Some(_a) = stack.pop() { }
        }
        StackOperation::Swap => {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                stack.push(a);
                stack.push(b);
            }
        }
        StackOperation::Over => {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                stack.push(b);
                stack.push(a);
                stack.push(b);
            }
        }
        StackOperation::Rotate => {
            if let (Some(a), Some(b), Some(c)) = (stack.pop(), stack.pop(), stack.pop()) {
                stack.push(b);
                stack.push(a);
                stack.push(c);
            }
        }
    }
}

fn execute_output_op(op: &OutputOperation, stack: &mut Stack) {
    match op {
        OutputOperation::Dot => {
            if let Some(a) = stack.pop() {
                println!("{a}");
            }
        }
        OutputOperation::Cr => {
            println!();
        }
        OutputOperation::Emit => {
            if let Some(a) = stack.pop() {
                let ascii = a as u8;
                print!("{}", ascii as char);
            }
        }
        OutputOperation::DotQuote(text) => {
            print!("{}", text);
        }
    }
}


fn execute_boolean_op(op: &BooleanOperation, stack: &mut Stack) {
    match op {
        BooleanOperation::Not => { //la separo porque es la unica que toma un valor (niega el último)
            if let Some(a) = stack.pop() {
                stack.push(if a == -1 { 0 } else { -1 });
            }
        },
        _ => {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) { // lo junto porque todas toman dos valores
                let result = match op {
                    BooleanOperation::Equal => a == b,
                    BooleanOperation::Greater => a < b,
                    BooleanOperation::Less => a > b,
                    BooleanOperation::And => a == -1 && b == -1,
                    BooleanOperation::Or => a == -1 || b == -1,
                    _ => false, //nunca deberia llegar acá
                };
                stack.push(if result { -1 } else { 0 });
            }
        }
    }
}

fn execute_conditional_op(op: &ConditionalOperation, stack: &mut Stack, execution_mode: &mut ExecutionMode) {
    match op {
        ConditionalOperation::If => {
            if let Some(condition) = stack.pop() {
                if condition == 0 {
                    *execution_mode = ExecutionMode::SkippingIf;
                } else {
                    *execution_mode = ExecutionMode::Executing;
                }
            } else {
                println!("Error: pila vacía en IF.");//stack underflow
            }
        }
        ConditionalOperation::Else => {
            match execution_mode {
                ExecutionMode::Executing => {
                    *execution_mode = ExecutionMode::SkippingElse;
                }
                ExecutionMode::SkippingIf => {
                    *execution_mode = ExecutionMode::Executing;
                }
                _ => {}
            }
        }
        ConditionalOperation::Then => {
            *execution_mode = ExecutionMode::Executing;
        }
    }
}



fn execute_other_operations(val: &ForthValue, stack: &mut Stack, dictionary: &WordsDictionary) {
    match val {
        ForthValue::Operation(ForthOperation::Arithmetic(op)) => execute_arithmetic_op(op, stack),
        ForthValue::Operation(ForthOperation::StackTypeOp(op)) => execute_stack_op(op, stack),
        ForthValue::Operation(ForthOperation::Output(op)) => execute_output_op(op, stack),
        ForthValue::Operation(ForthOperation::Boolean(op)) => execute_boolean_op(op, stack),
        ForthValue::Number(n) => stack.push(*n),
        ForthValue::Word(ForthWord::WordStart(word_name)) => {
            if let Some(definition) = dictionary.get_word(word_name) {
                let mut mode = ExecutionMode::Executing;
                for val in definition {
                    execute_instruction(val, stack, dictionary, &mut mode);
                }
            } else {
                println!("Error: palabra no definida '{}'", word_name);
            }
        }
        _ => { }
    }
}


fn execute_instruction(val: &ForthValue, stack: &mut Stack, dictionary: &WordsDictionary, execution_mode: &mut ExecutionMode) {
    match execution_mode {
        ExecutionMode::Executing => {
            match val {
                ForthValue::Operation(ForthOperation::Conditional(val)) => {
                    execute_conditional_op(val, stack, execution_mode)
                }
                ForthValue::Word(ForthWord::WordStart(word_name)) => {
                    if let Some(definition) = dictionary.get_word(word_name) {
                        let mut mode = ExecutionMode::Executing;
                        for val in definition {
                            execute_instruction(val, stack, dictionary, &mut mode);
                        }
                    } else { println!("Error: palabra no definida '{}'", word_name); }
                }
                _ => execute_other_operations(val, stack, dictionary),
            }
        }
        ExecutionMode::SkippingIf | ExecutionMode::SkippingElse => {
            match val {
                ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Then)) => {
                    *execution_mode = ExecutionMode::Executing;
                }
                ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Else)) => {
                    if *execution_mode == ExecutionMode::SkippingIf { *execution_mode = ExecutionMode::Executing; }
                    else { *execution_mode = ExecutionMode::SkippingElse; }
                }
                _ => { }
            }
        }
    }
}


fn handle_word_definition<'a>(tokens: &'a [String], i: &mut usize, flag: &mut bool, name: &mut &'a str, definition: &mut Vec<ForthValue>) {
    if *flag {
        println!("Error: ya se está definiendo una palabra.");
        return;
    }

    if *i + 1 >= tokens.len() {
        println!("Error: falta nombre de la palabra después de ':'");
        return;
    }

    *flag = true;
    *name = &tokens[*i + 1];
    definition.clear();
    *i += 1;
}


fn handle_word_end(flag: &mut bool, name: &str, definition: &mut Vec<ForthValue>, dictionary: &mut WordsDictionary) {
    if *flag {
        let mut new_definition = Vec::new();
        
        while let Some(item) = definition.pop() { 
            new_definition.insert(0, item);
        }
        
        dictionary.add_word(name, new_definition);
        *flag = false;
    } else {
        println!("Error: ';' sin inicio de definición.");
    }
}


fn handle_other_token(value: ForthValue, flag_defining_word: bool, definition: &mut Vec<ForthValue>, stack: &mut Stack, dictionary: &mut WordsDictionary) {
    if flag_defining_word {
        definition.push(value);
    } else {
        execute_instruction(&value, stack, dictionary, &mut ExecutionMode::Executing);
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
    
    while i < tokens.len() {
        let value = parse_token(&tokens[i]);

        match &value {
            ForthValue::Word(ForthWord::WordDefinition) => {
                handle_word_definition(tokens, &mut i, &mut flag_defining_word, &mut current_word_name, &mut current_definition);
            }
            ForthValue::Word(ForthWord::WordEnd) => {
                handle_word_end(&mut flag_defining_word, &current_word_name, &mut current_definition, dictionary);
            }
            _ => {
                handle_other_token(value, flag_defining_word, &mut current_definition, stack, dictionary);
            }
        }

        i += 1;
    }

    if flag_defining_word {
        println!("Error: falta ';'");
    }
}



fn main() {
    println!("----- Intérprete básico de Forth -----");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Error de lectura. Se espera el siguiente formato: cargo run -- ruta/a/main.fth [stack-size]");
        return;
    }

    println!("Argumentos: {args:#?}");
    let size: Option<&String> = args.get(2);
    let st = String::from(DEFAULT_STACK_SIZE_AS_STR);
    let size = size.unwrap_or(&st);
    let size: usize = match size.trim().parse() {
        Ok(sz) => sz,
        Err(_) => DEFAULT_STACK_SIZE
    };
    let mut stack = Stack::new(size);
    let mut words_dictionary = WordsDictionary::new();

    if let Ok(lines) = read_file(&args[1]) {
        for line in lines {
            let tokens = tokenize(&line);
            read_tokens(&tokens, &mut stack, &mut words_dictionary);
        }
    } else {
        println!("Error al leer el archivo .fth");
    }
}