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
            ForthOperation::Arithmetic(a) => {
                ForthOperation::Arithmetic(get_arithmetic_operation_value(a))
            }
            ForthOperation::StackTypeOp(s) => {
                ForthOperation::StackTypeOp(get_stack_operation_value(s))
            }
            ForthOperation::Output(o) => ForthOperation::Output(get_output_operation_value(o)),
            ForthOperation::Boolean(b) => ForthOperation::Boolean(get_boolean_operation_value(b)),
            ForthOperation::Conditional(c) => {
                ForthOperation::Conditional(get_conditional_operation_value(c))
            }
        }),
        ForthValue::Word(w) => ForthValue::Word(match w {
            ForthWord::Start(s) => ForthWord::Start(s.to_string()),
            ForthWord::Definition => ForthWord::Definition,
            ForthWord::End => ForthWord::End,
        }),
        ForthValue::Number(n) => ForthValue::Number(*n),
    }
}

fn get_arithmetic_operation_value(arithmetic_op: &ArithmeticOperation) -> ArithmeticOperation {
    match arithmetic_op {
        ArithmeticOperation::Add => ArithmeticOperation::Add,
        ArithmeticOperation::Substract => ArithmeticOperation::Substract,
        ArithmeticOperation::Multiply => ArithmeticOperation::Multiply,
        ArithmeticOperation::Divide => ArithmeticOperation::Divide,
    }
}

fn get_stack_operation_value(stack_op: &StackOperation) -> StackOperation {
    match stack_op {
        StackOperation::Duplicate => StackOperation::Duplicate,
        StackOperation::Drop => StackOperation::Drop,
        StackOperation::Swap => StackOperation::Swap,
        StackOperation::Over => StackOperation::Over,
        StackOperation::Rotate => StackOperation::Rotate,
    }
}

fn get_output_operation_value(output_op: &OutputOperation) -> OutputOperation {
    match output_op {
        OutputOperation::Dot => OutputOperation::Dot,
        OutputOperation::Emit => OutputOperation::Emit,
        OutputOperation::Cr => OutputOperation::Cr,
        OutputOperation::DotQuote(text) => OutputOperation::DotQuote(text.to_string()),
    }
}

fn get_boolean_operation_value(boolean_op: &BooleanOperation) -> BooleanOperation {
    match boolean_op {
        BooleanOperation::Equal => BooleanOperation::Equal,
        BooleanOperation::Less => BooleanOperation::Less,
        BooleanOperation::Greater => BooleanOperation::Greater,
        BooleanOperation::And => BooleanOperation::And,
        BooleanOperation::Or => BooleanOperation::Or,
        BooleanOperation::Not => BooleanOperation::Not,
    }
}

fn get_conditional_operation_value(conditional_op: &ConditionalOperation) -> ConditionalOperation {
    match conditional_op {
        ConditionalOperation::If => ConditionalOperation::If,
        ConditionalOperation::Then => ConditionalOperation::Then,
        ConditionalOperation::Else => ConditionalOperation::Else,
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
    let mut stack = crate::stack::Stack::new(TEST_STACK_SIZE);
    for &value in values_to_push {
        stack.push(value).unwrap();
    }
    stack
}
