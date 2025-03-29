use super::forth_operation::ForthOperation;
use crate::forth_value::ForthValue;

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
