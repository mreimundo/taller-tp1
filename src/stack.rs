use crate::errors::ForthError;
use std::{fs, io};

const STACK_REST_PATHNAME: &str = "stack.fth";

#[derive(Debug)]
pub struct Stack {
    pub data: Vec<i16>,
    max_elements: usize,
}

impl Stack {
    pub fn new(size: usize) -> Self {
        let max_elements = size / 2;
        Stack {
            data: Vec::with_capacity(max_elements),
            max_elements,
        }
    }

    pub fn push(&mut self, value: i16) -> Result<(), ForthError> {
        if self.data.len() >= self.max_elements {
            Err(ForthError::StackOverflow)
        } else {
            self.data.push(value);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<i16, ForthError> {
        self.data.pop().ok_or(ForthError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<&i16, ForthError> {
        self.data.last().ok_or(ForthError::StackUnderflow)
    }

    pub fn write_into_file(&mut self) -> io::Result<bool> {
        let stack_results: Vec<String> = self.data.iter().map(|&item| item.to_string()).collect();
        fs::write(STACK_REST_PATHNAME, stack_results.join(" "))?;
        Ok(true)
    }
}
