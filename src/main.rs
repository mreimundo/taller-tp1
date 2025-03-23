use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;
use std::str::Chars;

const DEFAULT_STACK_SIZE: usize = 1024 * 128; //128KB
const DEFAULT_STACK_SIZE_AS_STR: &'static str = "131072"; //128KB en string


/*-------------- TODO CHECKLIST --------------
    - Análisis de traits: qué tanto se pueden implementar y si lo vale o no
    - Check general de código y refactors: validar con las restricciones del TP y los requerimientos no funcionales. Ver nombre de tipos
    - Implementación de operaciones booleanas
    - Implementación de operaciones condicionales
    - Manejo de errores: implementación y pensar si vale la pena usar structs o algo de std
    - Separación en archivos
    - Tests: a la misma altura que src pero en módulos apartes tipo crate. Implementarlos usando #[cfg(test)] en c/u. No se testea main.rs
    - cargo fmt y cargo clippy
    - cargo doc
    - Probar compilación y ejecución en una distro de linux
*/

#[derive(Debug)]
enum Op {
    Arithmetic(ArithmeticOp),
    StackTypeOp(StackOp),
    WordTypeOp(WordOp),
    Output(OutputOp),
    Boolean(BooleanOp),
    Conditional(ConditionalOp),
    Number(i16),
    Unknown(String),
}

#[derive(Debug)]
enum ArithmeticOp {
    Add,
    Substract,
    Multiply,
    Divide
}

#[derive(Debug)]
enum StackOp {
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate
}

#[derive(Debug)]
enum WordOp {
    Word(String),
    WordDefinition,
    WordEnd
}

#[derive(Debug)]
enum OutputOp {
    Dot,
    Emit,
    Cr,
    DotQuote(String),
}

#[derive(Debug)]
enum BooleanOp {
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not,
}

#[derive(Debug)]
enum ConditionalOp {
    If,
    Then,
    Else,
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
    
    fn peek(&self) -> Option<i16> {
        self.0.last().copied()
    }
}

#[derive(Debug)]
struct WordsDictionary {
    words: HashMap<String, Vec<Op>>
}

impl WordsDictionary {
    fn new() -> Self {
        WordsDictionary {
            words: HashMap::new()
        }
    }
    
    fn add_word(&mut self, name: &str, definition: Vec<Op>) {
        self.words.insert(name.to_string(), definition);
    }
    
    fn get_word(&self, name: &str) -> Option<&Vec<Op>> {
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


/*recibe un token y devuelve la operación asociada al mismo.
si no encuentra una de las instrucciones básicas de forth, se trata de una word o de un número
*/
fn parse_token(token: &str) -> Op {
    if token.starts_with(".\"") {
        let quoted_text = &token[2..];
        return Op::Output(OutputOp::DotQuote(quoted_text.to_string()));
    }

    match token.to_uppercase().as_str() {
        "+" => Op::Arithmetic(ArithmeticOp::Add),
        "-" => Op::Arithmetic(ArithmeticOp::Substract),
        "*" => Op::Arithmetic(ArithmeticOp::Multiply),
        "/" => Op::Arithmetic(ArithmeticOp::Divide),

        "DUP" => Op::StackTypeOp(StackOp::Duplicate),
        "DROP" => Op::StackTypeOp(StackOp::Drop),
        "SWAP" => Op::StackTypeOp(StackOp::Swap),
        "OVER" => Op::StackTypeOp(StackOp::Over),
        "ROT" => Op::StackTypeOp(StackOp::Rotate),

        ":" => Op::WordTypeOp(WordOp::WordDefinition),
        ";" => Op::WordTypeOp(WordOp::WordEnd),

        "." => Op::Output(OutputOp::Dot),
        "EMIT" => Op::Output(OutputOp::Emit),
        "CR" => Op::Output(OutputOp::Cr),

        "=" => Op::Boolean(BooleanOp::Equal),
        "<" => Op::Boolean(BooleanOp::Less),
        ">" => Op::Boolean(BooleanOp::Greater),
        "AND" => Op::Boolean(BooleanOp::And),
        "OR" => Op::Boolean(BooleanOp::Or),
        "NOT" => Op::Boolean(BooleanOp::Not),

        "IF" => Op::Conditional(ConditionalOp::If),
        "THEN" => Op::Conditional(ConditionalOp::Then),
        "ELSE" => Op::Conditional(ConditionalOp::Else),

        _ => match token.parse::<i16>() {
            Ok(num) => Op::Number(num),
            Err(_) => Op::WordTypeOp(WordOp::Word(token.to_string()))
        }
    }
}


fn execute_arithmetic_op(op: &ArithmeticOp, stack: &mut Stack) {
    if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
        let result = match op {
            ArithmeticOp::Add => a + b,
            ArithmeticOp::Substract => b - a,
            ArithmeticOp::Multiply => a * b,
            ArithmeticOp::Divide => b / a, // Asume que a ≠ 0 (manejo de error pendiente)
        };
        println!("Resultado operación: {}", result);
        stack.push(result);
    }
}

fn execute_stack_op(op: &StackOp, stack: &mut Stack) {
    match op {
        StackOp::Duplicate => {
            if let Some(a) = stack.peek() {
                stack.push(a);
            }
        }
        StackOp::Drop => {
            if let Some(_a) = stack.pop() { }
        }
        StackOp::Swap => {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                stack.push(a);
                stack.push(b);
            }
        }
        StackOp::Over => {
            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                stack.push(b);
                stack.push(a);
                stack.push(b);
            }
        }
        StackOp::Rotate => {
            if let (Some(a), Some(b), Some(c)) = (stack.pop(), stack.pop(), stack.pop()) {
                stack.push(b);
                stack.push(a);
                stack.push(c);
            }
        }
    }
}

fn execute_output_op(op: &OutputOp, stack: &mut Stack) {
    match op {
        OutputOp::Dot => {
            if let Some(a) = stack.pop() {
                println!("{a}");
            }
        }
        OutputOp::Cr => {
            println!();
        }
        OutputOp::Emit => {
            if let Some(a) = stack.pop() {
                let ascii = a as u8;
                print!("{}", ascii as char);
            }
        }
        OutputOp::DotQuote(text) => {
            print!("{}", text);
        }
    }
}


fn not_done_yet () { println!("Not done yet."); }

fn execute_op(op: &Op, stack: &mut Stack, dictionary: &WordsDictionary) {
    match op {
        Op::Arithmetic(op) => execute_arithmetic_op(op, stack),
        Op::StackTypeOp(op) => execute_stack_op(op, stack),
        Op::Output(op) => execute_output_op(op, stack),
        Op::Boolean(_op) => not_done_yet(),
        Op::Conditional(_op) => not_done_yet(),
        Op::Number(n) => stack.push(*n),
        Op::WordTypeOp(WordOp::Word(word_name)) => {
            if let Some(definition) = dictionary.get_word(word_name) {
                for op in definition {
                    execute_op(op, stack, dictionary);
                }
            } else {
                println!("Error: palabra no definida '{}'", word_name);
            }
        }
        Op::WordTypeOp(WordOp::WordDefinition) | Op::WordTypeOp(WordOp::WordEnd) => { },
        Op::Unknown(op) => println!("Error: operación desconocida '{}'", op),
    }
}

/*función para leer los tokens.
realizando pattern matching se verifica si es una word, cuyo comportamiento se define aparte de las operaciones convencionales.
si no es de ese tipo o bien puede ejecutarse o está en medio de la definición de la word, por lo que se agrega el flag_defining_word
*/
fn read_tokens(tokens: &[String], stack: &mut Stack, dictionary: &mut WordsDictionary) {
    let mut i = 0;
    let mut flag_defining_word = false;
    let mut current_word_name = String::new();
    let mut current_definition = Vec::new();
    
    while i < tokens.len() {
        let operation = parse_token(&tokens[i]);

        match &operation {
            Op::WordTypeOp(WordOp::WordDefinition) => {
                if flag_defining_word {
                    println!("Error: ya se está definiendo una palabra.");
                    break;
                }

                if i + 1 >= tokens.len() {
                    println!("Error: falta nombre de la palabra después de ':'");
                    break;
                }

                flag_defining_word = true;
                current_word_name = tokens[i + 1].to_string();
                current_definition.clear();
                i += 1;
            }
            Op::WordTypeOp(WordOp::WordEnd) => {
                if flag_defining_word {
                    dictionary.add_word(&current_word_name, current_definition);
                    flag_defining_word = false;
                    current_definition = Vec::new();
                } else {
                    println!("Error: ';' sin inicio de definición.");
                }
            }
            _ => {
                if flag_defining_word {
                    current_definition.push(operation);
                } else {
                    execute_op(&operation, stack, dictionary); //no es word por lo tanto ejecuta la instrucción de una
                }
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