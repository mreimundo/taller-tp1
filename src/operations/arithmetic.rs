use super::forth_operation::ForthOperation;
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    stack::Stack,
};

/// Enum that represents the arithmetic operations that can be interpreted by the program.
///
/// The different ones are:
///
/// - Add: sum between two numeric elements.
/// - Substract: substract between two numeric elements.
/// - Multiply: multiplication between two numeric elements.
/// - Divide: division between two numeric elements.
///

#[derive(Debug)]
pub enum ArithmeticOperation {
    Add,
    Substract,
    Multiply,
    Divide,
}

///Function which converts a token received by parameter as &str to a ForthValue if exists, or None if not.

pub fn parse_arithmetic(token: &str) -> Option<ForthValue> {
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

///Function that executes an arithmetic operation by receiving a reference to an arithmetic operation and the stack reference as mutable so it can be updated with the result.

pub fn execute_arithmetic_op(op: &ArithmeticOperation, stack: &mut Stack) {
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

#[cfg(test)]
mod tests {
    use crate::utils::init_stack;

    use super::{ArithmeticOperation, execute_arithmetic_op};

    #[test]
    fn test_add_2() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_arithmetic_op(&ArithmeticOperation::Add, &mut test_stack);
        assert_eq!(test_stack.data, &[3]);
    }

    #[test]
    fn test_add_3() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_arithmetic_op(&ArithmeticOperation::Add, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 5]);
    }

    #[test]
    fn test_sub_2() {
        let mut test_stack = init_stack(&[3, 4]);
        execute_arithmetic_op(&ArithmeticOperation::Substract, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_sub_3() {
        let mut test_stack = init_stack(&[1, 12, 3]);
        execute_arithmetic_op(&ArithmeticOperation::Substract, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 9]);
    }

    #[test]
    fn test_mul_2() {
        let mut test_stack = init_stack(&[2, 4]);
        execute_arithmetic_op(&ArithmeticOperation::Multiply, &mut test_stack);
        assert_eq!(test_stack.data, &[8]);
    }

    #[test]
    fn test_mul_3() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_arithmetic_op(&ArithmeticOperation::Multiply, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 6]);
    }

    #[test]
    fn test_divide_2() {
        let mut test_stack = init_stack(&[12, 3]);
        execute_arithmetic_op(&ArithmeticOperation::Divide, &mut test_stack);
        assert_eq!(test_stack.data, &[4]);
    }

    #[test]
    fn test_divide_3() {
        let mut test_stack = init_stack(&[1, 12, 3]);
        execute_arithmetic_op(&ArithmeticOperation::Divide, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 4]);
    }
}
