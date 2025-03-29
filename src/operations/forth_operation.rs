use super::{
    arithmetic::ArithmeticOperation, boolean::BooleanOperation, conditional::ConditionalOperation,
    output::OutputOperation, stack_type::StackOperation,
};

#[derive(Debug)]
pub enum ForthOperation {
    Arithmetic(ArithmeticOperation),
    StackTypeOp(StackOperation),
    Output(OutputOperation),
    Boolean(BooleanOperation),
    Conditional(ConditionalOperation),
}
