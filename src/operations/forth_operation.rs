use super::{
    arithmetic::ArithmeticOperation, boolean::BooleanOperation, conditional::ConditionalOperation,
    output::OutputOperation, stack_type::StackOperation,
};

/// Enum that represents the different operations that can be interpreted by the program.
///
/// The possible operations are:
///
/// - Arithmetic: tuple that contains an arithmetic operation.
/// - StackTypeOp: tuple that contains a stack-type operation.
/// - Output: tuple that contains an output operation.
/// - Boolean: tuple that contains a boolean operation.
/// - Conditional: tuple that contains a conditional operation.
///

#[derive(Debug)]
pub enum ForthOperation {
    Arithmetic(ArithmeticOperation),
    StackTypeOp(StackOperation),
    Output(OutputOperation),
    Boolean(BooleanOperation),
    Conditional(ConditionalOperation),
}
