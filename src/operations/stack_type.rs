use super::forth_operation::ForthOperation;
use crate::{forth_value::ForthValue, errors::print_error, stack::Stack};

#[derive(Debug)]
pub enum StackOperation {
    Duplicate,
    Drop,
    Swap,
    Over,
    Rotate,
}

pub fn parse_stack_op(token: &str) -> Option<ForthValue> {
    match token {
        "DUP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Duplicate,
        ))),
        "DROP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Drop,
        ))),
        "SWAP" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Swap,
        ))),
        "OVER" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Over,
        ))),
        "ROT" => Some(ForthValue::Operation(ForthOperation::StackTypeOp(
            StackOperation::Rotate,
        ))),
        _ => None,
    }
}


pub fn execute_stack_op(op: &StackOperation, stack: &mut Stack) {
    match op {
        StackOperation::Duplicate => match stack.peek() {
            Ok(a) => {
                if let Err(e) = stack.push(*a) {
                    print_error(e);
                }
            }
            Err(e) => print_error(e),
        },
        StackOperation::Drop => {
            if let Err(e) = stack.pop() {
                print_error(e);
            }
        }
        StackOperation::Swap => {
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
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Over => {
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
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Rotate => {
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
            let c = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(c) {
                print_error(e);
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::{execute_stack_op, StackOperation};
    use crate::utils::init_stack;

    #[test]
    fn test_dup_1() {
        let mut test_stack = init_stack(&[1]);
        execute_stack_op(&StackOperation::Duplicate, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 1]);
    }

    #[test]
    fn test_dup_2() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_stack_op(&StackOperation::Duplicate, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 2, 2]);
    }

    #[test]
    fn test_drop_1() {
        let mut test_stack = init_stack(&[1]);
        execute_stack_op(&StackOperation::Drop, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_drop_2() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_stack_op(&StackOperation::Drop, &mut test_stack);
        assert_eq!(test_stack.data, &[1]);
    }

    #[test]
    fn test_swap_1() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_stack_op(&StackOperation::Swap, &mut test_stack);
        assert_eq!(test_stack.data, &[2, 1]);
    }

    #[test]
    fn test_swap_2() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_stack_op(&StackOperation::Swap, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 3, 2]);
    }

    #[test]
    fn test_over_1() {
        let mut test_stack = init_stack(&[1, 2]);
        execute_stack_op(&StackOperation::Over, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 2, 1]);
    }

    #[test]
    fn test_over_2() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_stack_op(&StackOperation::Over, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 2, 3, 2]);
    }

    #[test]
    fn test_rot_1() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_stack_op(&StackOperation::Rotate, &mut test_stack);
        assert_eq!(test_stack.data, &[2, 3, 1]);
    }

    #[test]
    fn test_rot_2() {
        let mut test_stack = init_stack(&[1, 2, 3]);
        execute_stack_op(&StackOperation::Rotate, &mut test_stack);
        execute_stack_op(&StackOperation::Rotate, &mut test_stack);
        execute_stack_op(&StackOperation::Rotate, &mut test_stack);
        assert_eq!(test_stack.data, &[1, 2, 3]);
    }

    #[test]
    fn test_drop_empty_stack() {
        let mut test_stack = init_stack(&[]);
        execute_stack_op(&StackOperation::Drop, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }

    #[test]
    fn test_dup_empty_stack() {
        let mut test_stack = init_stack(&[]);
        execute_stack_op(&StackOperation::Duplicate, &mut test_stack);
        assert_eq!(test_stack.data, &[]);
    }
}