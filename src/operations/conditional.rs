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
