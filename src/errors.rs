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
    Generic(String),
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
                "wrong-input. Try executing with format: cargo run -- path/to/main.fth stack-size=[size_of_stack]"
            ),
            ForthError::Generic(value) => write!(f, "[ERROR]: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        errors::{ForthError, print_error},
        operations::{
            arithmetic::{ArithmeticOperation, execute_arithmetic_op},
            stack_type::{StackOperation, execute_stack_op},
        },
        stack::Stack,
        tokens::{read_tokens, tokenize},
        utils::init_stack,
        words::dictionary::WordsDictionary,
    };

    #[test]
    fn test_arithmetic_underflows() {
        //pruebo las operaciones aritméticas, con todas teniendo solo el operador o un número y el operador, debería tirar "stack-underflow" (y stack vacío)
        let ops = [
            ArithmeticOperation::Add,
            ArithmeticOperation::Substract,
            ArithmeticOperation::Multiply,
            ArithmeticOperation::Divide,
        ];

        for op in ops {
            let mut stack = Stack::new(10); //inicializacion sin valores a pushear
            execute_arithmetic_op(&op, &mut stack);
            assert!(stack.data.is_empty());

            let mut stack = init_stack(&[1]);
            execute_arithmetic_op(&op, &mut stack);
            assert!(stack.data.is_empty());
        }
    }

    #[test]
    fn test_stack_op_underflows() {
        //pruebo las operaciones de stack, con todas teniendo solo la operación deberia arrojar "stack-underflow" (y stack vacío)
        let ops = [
            StackOperation::Duplicate,
            StackOperation::Drop,
            StackOperation::Swap,
            StackOperation::Over,
            StackOperation::Rotate,
        ];

        for op in ops {
            let mut test_stack = Stack::new(10); //inicializacion sin valores a pushear
            execute_stack_op(&op, &mut test_stack);
            assert!(test_stack.data.is_empty());
        }

        //hay algunas operaciones de stack que con un elemento también generan stack-underflow, como swap y over
        let mut test_stack = init_stack(&[1]);
        execute_stack_op(&StackOperation::Swap, &mut test_stack);
        assert!(test_stack.data.is_empty());

        let mut test_stack = init_stack(&[1]);
        execute_stack_op(&StackOperation::Over, &mut test_stack);
        assert!(test_stack.data.is_empty());
    }

    #[test]
    fn test_division_by_zero() {
        let mut test_stack = init_stack(&[4, 0]);
        execute_arithmetic_op(&ArithmeticOperation::Divide, &mut test_stack);
        assert!(test_stack.data.is_empty());
    }

    #[test]
    fn test_stack_overflow() {
        let stack_size_bytes = 10;
        let mut test_stack = Stack::new(stack_size_bytes);

        for i in 1..=5 {
            test_stack.push(i).expect("Debería aceptar estos valores");
        }

        match test_stack.push(6) {
            Err(ForthError::StackOverflow) => print_error(ForthError::StackOverflow),
            Ok(_) => panic!(),
            Err(_e) => panic!(),
        }

        assert_eq!(test_stack.data, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_invalid_word_number() {
        let mut dict = WordsDictionary::new();
        let mut stack = Stack::new(100);

        read_tokens(&tokenize(": 1 2 ;"), &mut stack, &mut dict);
        assert!(stack.data.is_empty());
    }

    #[test]
    fn test_unknown_word() {
        let mut dict = WordsDictionary::new();
        let mut stack = Stack::new(100);

        read_tokens(&tokenize("foo"), &mut stack, &mut dict);
        assert!(stack.data.is_empty());
    }
}
