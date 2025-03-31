use crate::errors::ForthError;
use std::{fs, io};

const STACK_REST_PATHNAME: &str = "stack.fth";

/// This struct is the main of the program. It is used everywhere to save and get an element.
/// It contains data (as pub so it can be accessed by other modules) of i16 values. The max_elements field is used to initialize the struct by the size received in the program input

#[derive(Debug)]
pub struct Stack {
    pub data: Vec<i16>,
    max_elements: usize,
}

impl Stack {
    /// Function used to build the structure. Receives a size that sets the max_elements of the Stack

    pub fn new(size: usize) -> Self {
        let max_elements = size / 2;
        Stack {
            data: Vec::with_capacity(max_elements),
            max_elements,
        }
    }

    /// Function to add an i16 value to the top of the Stack. Returns Ok if possible or stack-overflow error if it exceeds the structure max_elements.

    pub fn push(&mut self, value: i16) -> Result<(), ForthError> {
        if self.data.len() >= self.max_elements {
            Err(ForthError::StackOverflow)
        } else {
            self.data.push(value);
            Ok(())
        }
    }

    /// Function to get the top i16 value of the Stack. Returns the value if possible or stack-underflow if the element does not exists.

    pub fn pop(&mut self) -> Result<i16, ForthError> {
        self.data.pop().ok_or(ForthError::StackUnderflow)
    }

    /// Function to get a reference of the top i16 value of the Stack. Returns a reference to the value if possible or stack-underflow if the element does not exists.
    /// Unlike pop, peek does not mutate the stack.

    pub fn peek(&self) -> Result<&i16, ForthError> {
        self.data.last().ok_or(ForthError::StackUnderflow)
    }

    /// Function used to write the rest of the stack to a stack.fth file. Returns Ok(true), letting the error be handled by the function that call it.

    pub fn write_into_file(&mut self) -> io::Result<bool> {
        let stack_results: Vec<String> = self.data.iter().map(|&item| item.to_string()).collect();
        fs::write(STACK_REST_PATHNAME, stack_results.join(" "))?;
        Ok(true)
    }
}
