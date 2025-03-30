use super::forth_operation::ForthOperation;
use crate::{forth_value::ForthValue, other_executions::ExecutionStage, errors::print_error, stack::Stack};

#[derive(Debug)]
pub enum ConditionalOperation {
    If,
    Then,
    Else,
}

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


pub fn execute_conditional_op(
    op: &ConditionalOperation,
    stack: &mut Stack,
    execution_mode: &mut Vec<ExecutionStage>,
) {
    match op {
        ConditionalOperation::If => match stack.pop() {
            Ok(condition) => {
                if condition == 0 {
                    execution_mode.push(ExecutionStage::Skipping(1));
                } else {
                    execution_mode.push(ExecutionStage::Executing);
                }
            }
            Err(e) => print_error(e),
        },
        ConditionalOperation::Else => {
            if let Some(last) = execution_mode.last_mut() {
                match last {
                    ExecutionStage::Executing => {
                        *last = ExecutionStage::Skipping(1);
                    }
                    ExecutionStage::Skipping(depth) => {
                        if *depth == 1 {
                            if let Some(mode) = execution_mode.last_mut() {
                                *mode = ExecutionStage::Executing;
                            }
                        }
                    }
                }
            }
        }
        ConditionalOperation::Then => {
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
    }
}



#[cfg(test)]
mod tests {
    use crate::{
        tokens::{read_tokens, tokenize},
        stack::Stack,
        words::dictionary::WordsDictionary
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
        
        read_tokens(&tokenize(": f if 2 else 3 then ;"), &mut test_stack, &mut dict);
        
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
        
        read_tokens(&tokenize(": f if if 1 else 2 then else drop 3 then ;"), &mut test_stack, &mut dict);
        
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
        
        read_tokens(&tokenize(": f dup 0 = if drop 2 else dup 1 = if drop 3 else drop 4 then then ;"), &mut test_stack, &mut dict);
        
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