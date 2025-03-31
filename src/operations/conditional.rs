use super::forth_operation::ForthOperation;
use crate::{
    errors::print_error, forth_value::ForthValue, other_executions::ExecutionStage, stack::Stack,
};

/// Enum that represents the conditional operations that can be interpreted by the program.
///
/// The different ones are:
///
/// - If: evaluates an expression.
/// - Then: end of the condition, which can execute an instruction.
/// - Else: execute an instruction by knowing the if condition evaluates to false.
///

#[derive(Debug)]
pub enum ConditionalOperation {
    If,
    Then,
    Else,
}

///Function which converts a token received by parameter as &str to a ForthValue if exists, or None if not.
pub fn parse_conditional(token: &str) -> Option<ForthValue> {
    match token {
        "IF" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::If,
        ))),
        "THEN" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::Then,
        ))),
        "ELSE" => Some(ForthValue::Operation(ForthOperation::Conditional(
            ConditionalOperation::Else,
        ))),
        _ => None,
    }
}

///Function that executes a conditional operation by receiving a reference to a conditional operation and the stack reference as mutable so it can be updated with the result.
pub fn execute_conditional_op(
    op: &ConditionalOperation,
    stack: &mut Stack,
    execution_mode: &mut Vec<ExecutionStage>,
) {
    match op {
        ConditionalOperation::If => handle_if(stack, execution_mode),
        ConditionalOperation::Else => handle_else(execution_mode),
        ConditionalOperation::Then => handle_then(execution_mode),
    }
}

fn handle_if(stack: &mut Stack, execution_mode: &mut Vec<ExecutionStage>) {
    match stack.pop() {
        Ok(condition) => {
            let stage = if condition == 0 {
                ExecutionStage::Skipping(1)
            } else {
                ExecutionStage::Executing
            };
            execution_mode.push(stage);
        }
        Err(e) => print_error(e),
    }
}

fn handle_else(execution_mode: &mut [ExecutionStage]) {
    if let Some(last) = execution_mode.last_mut() {
        match last {
            ExecutionStage::Executing => {
                *last = ExecutionStage::Skipping(1);
            }
            ExecutionStage::Skipping(depth) if *depth == 1 => {
                if let Some(mode) = execution_mode.last_mut() {
                    *mode = ExecutionStage::Executing;
                }
            }
            _ => {}
        }
    }
}

fn handle_then(execution_mode: &mut Vec<ExecutionStage>) {
    if let Some(last) = execution_mode.last_mut() {
        match last {
            ExecutionStage::Skipping(depth) => {
                if *depth > 1 {
                    *depth -= 1;
                } else {
                    execution_mode.pop();
                }
            }
            _ => {
                execution_mode.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        stack::Stack,
        tokens::{read_tokens, tokenize},
        words::dictionary::WordsDictionary,
    };

    #[test]
    fn test_if_simple() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": f if 2 then ;"), &mut test_stack, &mut dict);

        test_stack.push(-1).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[2]);
    }

    #[test]
    fn test_if_else() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(
            &tokenize(": f if 2 else 3 then ;"),
            &mut test_stack,
            &mut dict,
        );

        test_stack.push(-1).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        test_stack.push(0).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[2, 3]);
    }

    #[test]
    fn test_nested_if_inline() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(
            &tokenize(": f if if 1 else 2 then else drop 3 then ;"),
            &mut test_stack,
            &mut dict,
        );

        test_stack.push(-1).unwrap();
        test_stack.push(-1).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        test_stack.push(0).unwrap();
        test_stack.push(-1).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        test_stack.push(0).unwrap();
        test_stack.push(0).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[1, 2, 3]);
    }

    #[test]
    fn test_nested_if_else() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(
            &tokenize(": f dup 0 = if drop 2 else dup 1 = if drop 3 else drop 4 then then ;"),
            &mut test_stack,
            &mut dict,
        );

        test_stack.push(0).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        test_stack.push(1).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        test_stack.push(2).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[2, 3, 4]);
    }

    #[test]
    fn test_if_non_canonical() {
        let mut dict = WordsDictionary::new();
        let mut test_stack = Stack::new(100);

        read_tokens(&tokenize(": f if 10 then ;"), &mut test_stack, &mut dict);

        test_stack.push(5).unwrap();
        read_tokens(&tokenize("f"), &mut test_stack, &mut dict);

        assert_eq!(test_stack.data, &[10]);
    }
}
