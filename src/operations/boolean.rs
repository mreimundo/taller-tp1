use super::forth_operation::ForthOperation;
use crate::{forth_value::ForthValue, stack::Stack, errors::{ForthError, print_error}};

#[derive(Debug)]
pub enum BooleanOperation {
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not,
}

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


pub fn execute_boolean_op(op: &BooleanOperation, stack: &mut Stack) {
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
                    print_error(ForthError::Generic("Unknown boolean operation".to_string()));
                    false
                }
            };
            if let Err(e) = stack.push(if result { -1 } else { 0 }) {
                print_error(e);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{execute_boolean_op, BooleanOperation};
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