use super::forth_operation::ForthOperation;
use crate::{forth_value::ForthValue, errors::print_error, stack::Stack};

#[derive(Debug)]
pub enum OutputOperation {
    Dot,
    Emit,
    Cr,
    DotQuote(String),
}

pub fn parse_output(token: &str) -> Option<ForthValue> {
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
        _ => None, //DotQuote (para imprimir por pantalla) lo manejamos aparte
    }
}


pub fn execute_output_op(op: &OutputOperation, stack: &mut Stack) {
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

#[cfg(test)]
//observacion: sobre estos no se testeo explícitamente que la salida sea la misma, dado que esto se ve reflejado al ejecutar el test
mod tests {
    use super::{execute_output_op, OutputOperation};
    use crate::utils::init_stack;
    #[test]
    fn test_dot_without_leftover() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_with_leftover() {
        let mut test_stack = init_stack(&[1, 2, 3, 4, 5]);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 2]);
    }

    #[test]
    fn test_cr_1() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_cr_2() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_and_cr() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        execute_output_op(&OutputOperation::Dot, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_emit_uppercase() {
        let mut test_stack = init_stack(&[65]);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_emit_lowercase() {
        let mut test_stack = init_stack(&[97]);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_emit_multiple() {
        let mut test_stack = init_stack(&[68, 67, 66, 65]);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        execute_output_op(&OutputOperation::Emit, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_quote_hello_world() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::DotQuote("hello world".to_string()), &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_quote_multiple_whitespace() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::DotQuote("hello      world!".to_string()), &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_quote_multiples() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::DotQuote("hello".to_string()), &mut test_stack);
        execute_output_op(&OutputOperation::DotQuote("world".to_string()), &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dot_quote_and_cr() {
        let mut test_stack = init_stack(&[]);
        execute_output_op(&OutputOperation::DotQuote("hello".to_string()), &mut test_stack);
        execute_output_op(&OutputOperation::Cr, &mut test_stack);
        execute_output_op(&OutputOperation::DotQuote("world".to_string()), &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }
}