pub mod errors;
pub mod forth_value;
pub mod operations;
pub mod other_executions;
pub mod stack;
pub mod tokens;
pub mod utils;
pub mod words;

pub use errors::{ForthError, print_error};
pub use stack::Stack;
use tokens::{read_tokens, tokenize};
use utils::read_file;
use words::dictionary::WordsDictionary;

const DEFAULT_STACK_SIZE: usize = 1024 * 128; //128KB


fn interpret_forth_file(filename: &str, stack: &mut Stack, dictionary: &mut WordsDictionary) {
    match read_file(filename) {
        Ok(lines) => {
            for line in lines {
                let tokens = tokenize(&line);
                read_tokens(&tokens, stack, dictionary);
            }

            match stack.write_into_file() {
                Ok(_) => println!("Stack ({:?}) written in stack.fth!", stack.data),
                Err(_) => print_error(ForthError::Generic("Impossible to write stack".to_string())),
            }
        }
        Err(_) => print_error(ForthError::Generic(
            "Impossible to read file.fth".to_string(),
        )),
    }
}

fn main() {
    println!("----- Basic Forth-79 Interpreter -----");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 3 || (args.len() == 3 && !args[2].starts_with("stack-size=")) {
        print_error(ForthError::WrongInput);
        return;
    }

    let size_bytes = args
        .iter()
        .find(|arg| arg.starts_with("stack-size="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(DEFAULT_STACK_SIZE);

    let mut stack = Stack::new(size_bytes);
    let mut words_dictionary = WordsDictionary::new();

    interpret_forth_file(&args[1], &mut stack, &mut words_dictionary);
}
