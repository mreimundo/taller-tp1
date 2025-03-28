use std::fmt::Display;

/* Enum que representa los errores que puede tener la ejecución del intérprete de Forth, ante un valor determinado
Los distintos son:
- StackUnderflow: cuando una operación intenta popear un elemento de una pila vacía.
- StackOverflow: cuando una operación intenta pushear un elemento a una pila que se encuentra en su capacidad máxima de memoria.
- InvalidWord: cuando se trata de definir una word invalida, por ejemplo: : 1 1 ;.
- DivisionByZero: cuando se trata de dividir por cero.
- UnknownWord: cuando el interprete no puede hallar la definición de la word evaluada.
- WrongInput: cuando el formato con el cual se ejecuta el programa es erróneo.
- Generic: generic type for other possible errors detected.
*/

#[derive(Debug, PartialEq)]
pub enum ForthError {
    StackUnderflow,
    StackOverflow,
    InvalidWord,
    DivisionByZero,
    UnknownWord,
    WrongInput,
    Generic(&'static str),
}

pub fn print_error(error: ForthError) {
    println!("{}", error);
}

impl Display for ForthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForthError::StackUnderflow => write!(f, "stack-underflow"),
            ForthError::StackOverflow => write!(f, "stack-overflow"),
            ForthError::InvalidWord => write!(f, "invalid-word"),
            ForthError::DivisionByZero => write!(f, "division-by-zero"),
            ForthError::UnknownWord => write!(f, "?"),
            ForthError::WrongInput => write!(
                f,
                "wrong-input. Try executing with format: cargo run -- path/to/main.fth [stack-size]"
            ),
            ForthError::Generic(value) => write!(f, "[ERROR]: {}", value.to_string()),
        }
    }
}
