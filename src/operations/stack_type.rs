use super::forth_operation::ForthOperation;
use crate::forth_value::ForthValue;

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
