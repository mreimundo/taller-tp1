use super::forth_operation::ForthOperation;
use crate::forth_value::ForthValue;

#[derive(Debug)]
pub enum OutputOperation {
    Dot,
    Emit,
    Cr,
    DotQuote(String),
}

pub fn parse_output(token: &str) -> Option<ForthValue> {
    match token {
        "." => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Dot,
        ))),
        "EMIT" => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Emit,
        ))),
        "CR" => Some(ForthValue::Operation(ForthOperation::Output(
            OutputOperation::Cr,
        ))),
        _ => None,
    }
}
