use crate::{operations::forth_operation::ForthOperation, words::word::ForthWord};

#[derive(Debug)]
pub enum ForthValue {
    Operation(ForthOperation),
    Word(ForthWord),
    Number(i16),
}
