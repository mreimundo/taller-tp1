use crate::operations::{
    arithmetic::ArithmeticOperation, boolean::BooleanOperation, conditional::ConditionalOperation,
    forth_operation::ForthOperation, output::OutputOperation, stack_type::StackOperation,
};
use crate::{forth_value::ForthValue, words::word::ForthWord};
use std::fs::File;
use std::io::{self, BufRead};

pub fn get_copy_forth_value(value: &ForthValue) -> ForthValue {
    match value {
        ForthValue::Operation(op) => ForthValue::Operation(match op {
            ForthOperation::Arithmetic(a) => ForthOperation::Arithmetic(match a {
                ArithmeticOperation::Add => ArithmeticOperation::Add,
                ArithmeticOperation::Substract => ArithmeticOperation::Substract,
                ArithmeticOperation::Multiply => ArithmeticOperation::Multiply,
                ArithmeticOperation::Divide => ArithmeticOperation::Divide,
            }),
            ForthOperation::StackTypeOp(s) => ForthOperation::StackTypeOp(match s {
                StackOperation::Duplicate => StackOperation::Duplicate,
                StackOperation::Drop => StackOperation::Drop,
                StackOperation::Swap => StackOperation::Swap,
                StackOperation::Over => StackOperation::Over,
                StackOperation::Rotate => StackOperation::Rotate,
            }),
            ForthOperation::Output(o) => ForthOperation::Output(match o {
                OutputOperation::Dot => OutputOperation::Dot,
                OutputOperation::Emit => OutputOperation::Emit,
                OutputOperation::Cr => OutputOperation::Cr,
                OutputOperation::DotQuote(text) => OutputOperation::DotQuote(text.to_string()),
            }),
            ForthOperation::Boolean(b) => ForthOperation::Boolean(match b {
                BooleanOperation::Equal => BooleanOperation::Equal,
                BooleanOperation::Less => BooleanOperation::Less,
                BooleanOperation::Greater => BooleanOperation::Greater,
                BooleanOperation::And => BooleanOperation::And,
                BooleanOperation::Or => BooleanOperation::Or,
                BooleanOperation::Not => BooleanOperation::Not,
            }),
            ForthOperation::Conditional(c) => ForthOperation::Conditional(match c {
                ConditionalOperation::If => ConditionalOperation::If,
                ConditionalOperation::Then => ConditionalOperation::Then,
                ConditionalOperation::Else => ConditionalOperation::Else,
            }),
        }),
        ForthValue::Word(w) => ForthValue::Word(match w {
            ForthWord::Start(s) => ForthWord::Start(s.to_string()),
            ForthWord::Definition => ForthWord::Definition,
            ForthWord::End => ForthWord::End,
        }),
        ForthValue::Number(n) => ForthValue::Number(*n),
    }
}

pub fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

//a fines de test, una función para inicializar el stack
#[cfg(test)]
const TEST_STACK_SIZE: usize = 1024 * 128;

#[cfg(test)]
pub fn init_stack(values_to_push: &[i16]) -> crate::stack::Stack {
    //me aparece error si pongo el use en cualquier lugar arriba de esta línea
    let mut stack = crate::stack::Stack::new(TEST_STACK_SIZE);
    for &value in values_to_push {
        stack.push(value).unwrap();
    }
    stack
}
