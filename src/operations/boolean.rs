use super::forth_operation::ForthOperation;
use crate::forth_value::ForthValue;

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
