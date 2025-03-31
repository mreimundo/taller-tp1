use crate::{operations::forth_operation::ForthOperation, words::word::ForthWord};

/// Enum that represents the values that can be interpreted by the program.
///
/// The different values are:
///
/// - Operation: tuple that contains a ForthOperation.
/// - Word: tuple that contains a ForthOperation.
/// - Number: tuple that contains an i16 number.
///

#[derive(Debug)]
pub enum ForthValue {
    Operation(ForthOperation),
    Word(ForthWord),
    Number(i16),
}
