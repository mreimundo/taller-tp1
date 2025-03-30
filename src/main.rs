mod errors;
mod other_executions;
mod forth_value;
mod operations;
mod stack;
mod tokens;
mod utils;
mod words;

use errors::{ForthError, print_error};
use stack::Stack;
use tokens::{read_tokens, tokenize};
use utils::read_file;
use words::dictionary::WordsDictionary;

const DEFAULT_STACK_SIZE: usize = 1024 * 128; //128KB

/*-------------- TODO CHECKLIST --------------
- Tests: a la misma altura que src pero en módulos apartes tipo crate. Implementarlos usando #[cfg(test)] en c/u. No se testea main.rs
- completar cargo doc
- Probar compilación y ejecución en una distro de linux. Validar todos los casos posibles
- Reducir líneas de algunas funciones
- Corregir nested ifs
*/

fn interpret_forth_file(filename: &str, stack: &mut Stack, dictionary: &mut WordsDictionary) {
    match read_file(filename) {
        Ok(lines) => {
            for line in lines {
                let tokens = tokenize(&line);
                read_tokens(&tokens, stack, dictionary);
            }

            match stack.write_into_file() {
                Ok(_) => println!("Stack restante ({:?}) escrito en stack.fth!", stack.data),
                Err(_) => print_error(ForthError::Generic("Impossible to write stack".to_string())),
            }
        }
        Err(_) => print_error(ForthError::Generic(
            "Impossible to read file.fth".to_string(),
        )),
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

    interpret_forth_file(&args[1], &mut stack, &mut words_dictionary);
}
