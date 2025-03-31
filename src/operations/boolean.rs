use super::forth_operation::ForthOperation;
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    stack::Stack,
};

/// Enum that represents the boolean operations that can be interpreted by the program.
///
/// The different ones are:
///
/// - Equal: checks if two numeric are equal.
/// - Less: checks if one numeric element is less than the other one.
/// - Greater: checks if one numeric element is greater than the other one.
/// - And: checks if the condition between two expressions is true.
/// - Or: checks if the condition either of two expressions is true.
/// - Not: Denies a value.
///

#[derive(Debug)]
pub enum BooleanOperation {
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not,
}

///Function which converts a token received by parameter as &str to a ForthValue if exists, or None if not.

pub fn parse_boolean(token: &str) -> Option<ForthValue> {
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

///Function that executes a boolean operation by receiving a reference to a boolean operation and the stack reference as mutable so it can be updated with the result.
pub fn execute_boolean_op(op: &BooleanOperation, stack: &mut Stack) {
    match op {
        BooleanOperation::Not => handle_not_operation(stack),
        _ => handle_other_boolean_ops(op, stack),
    }
}

fn handle_not_operation(stack: &mut Stack) {
    match stack.pop() {
        Ok(a) => {
            let result = if a != 0 { 0 } else { -1 };
            push_boolean_result(stack, result);
        }
        Err(e) => print_error(e),
    }
}

fn handle_other_boolean_ops(op: &BooleanOperation, stack: &mut Stack) {
    let (a, b) = match (stack.pop(), stack.pop()) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(e), _) | (_, Err(e)) => {
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
            print_error(ForthError::Generic("Unknown boolean operation".to_string()));
            false
        }
    };

    push_boolean_result(stack, if result { -1 } else { 0 });
}

fn push_boolean_result(stack: &mut Stack, value: i16) {
    if let Err(e) = stack.push(value) {
        print_error(e);
    }
}

#[cfg(test)]
mod tests {
    use super::{BooleanOperation, execute_boolean_op};
    use crate::utils::init_stack;

    #[test]
    fn test_equals_true() {
        let mut test_stack = init_stack(&[1, 1]);
        execute_boolean_op(&BooleanOperation::Equal, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_equals_false() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_boolean_op(&BooleanOperation::Equal, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_less_true() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_boolean_op(&BooleanOperation::Less, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_less_false() {
        let mut test_stack = init_stack(&[2, 1]);
        execute_boolean_op(&BooleanOperation::Less, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_less_equals() {
        let mut test_stack = init_stack(&[2, 2]);
        execute_boolean_op(&BooleanOperation::Less, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_greater_true() {
        let mut test_stack = init_stack(&[2, 1]);
        execute_boolean_op(&BooleanOperation::Greater, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_greater_false() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_boolean_op(&BooleanOperation::Greater, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_greater_equals() {
        let mut test_stack = init_stack(&[2, 2]);
        execute_boolean_op(&BooleanOperation::Greater, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_and_none() {
        let mut test_stack = init_stack(&[0, 0]);
        execute_boolean_op(&BooleanOperation::And, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_and_one() {
        let mut test_stack = init_stack(&[-1, 0]);
        execute_boolean_op(&BooleanOperation::And, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_and_both() {
        let mut test_stack = init_stack(&[-1, -1]);
        execute_boolean_op(&BooleanOperation::And, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_or_none() {
        let mut test_stack = init_stack(&[0, 0]);
        execute_boolean_op(&BooleanOperation::Or, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_or_one() {
        let mut test_stack = init_stack(&[-1, 0]);
        execute_boolean_op(&BooleanOperation::Or, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_or_both() {
        let mut test_stack = init_stack(&[-1, -1]);
        execute_boolean_op(&BooleanOperation::Or, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_not_true() {
        let mut test_stack = init_stack(&[-1]);
        execute_boolean_op(&BooleanOperation::Not, &mut test_stack);
        assert_eq!(test_stack.data, &[0]);
    }

    #[test]
    fn test_not_false() {
        let mut test_stack = init_stack(&[0]);
        execute_boolean_op(&BooleanOperation::Not, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }

    #[test]
    fn test_not_not() {
        let mut test_stack = init_stack(&[10]);
        execute_boolean_op(&BooleanOperation::Not, &mut test_stack);
        execute_boolean_op(&BooleanOperation::Not, &mut test_stack);
        assert_eq!(test_stack.data, &[-1]);
    }
}
