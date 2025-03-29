use super::forth_operation::ForthOperation;
use crate::forth_value::ForthValue;

#[derive(Debug)]
pub enum ArithmeticOperation {
    Add,
    Substract,
    Multiply,
    Divide,
}

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
