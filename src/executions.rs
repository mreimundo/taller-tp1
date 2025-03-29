use crate::operations::{
    arithmetic::ArithmeticOperation, boolean::BooleanOperation, conditional::ConditionalOperation,
    forth_operation::ForthOperation, output::OutputOperation, stack_type::StackOperation,
};
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    stack::Stack,
    words::{dictionary::WordsDictionary, word::ForthWord},
};

#[derive(PartialEq)]
pub enum ExecutionStage {
    Executing,
    Skipping(usize),
}

pub fn execute_arithmetic_op(op: &ArithmeticOperation, stack: &mut Stack) {
    let a = match stack.pop() {
        Ok(val) => val,
        Err(e) => {
            print_error(e);
            return;
        }
    };
    let b = match stack.pop() {
        Ok(val) => val,
        Err(e) => {
            print_error(e);
            return;
        }
    };

    let result = match op {
        ArithmeticOperation::Add => a + b,
        ArithmeticOperation::Substract => b - a,
        ArithmeticOperation::Multiply => a * b,
        ArithmeticOperation::Divide => {
            if a != 0 {
                b / a
            } else {
                print_error(ForthError::DivisionByZero);
                return;
            }
        }
    };

    if let Err(e) = stack.push(result) {
        print_error(e);
    }
}

pub fn execute_stack_op(op: &StackOperation, stack: &mut Stack) {
    match op {
        StackOperation::Duplicate => match stack.peek() {
            Ok(a) => {
                if let Err(e) = stack.push(*a) {
                    print_error(e);
                }
            }
            Err(e) => print_error(e),
        },
        StackOperation::Drop => {
            if let Err(e) = stack.pop() {
                print_error(e);
            }
        }
        StackOperation::Swap => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Over => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(b) {
                print_error(e);
            }
        }
        StackOperation::Rotate => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let c = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            if let Err(e) = stack.push(b) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(a) {
                print_error(e);
                return;
            }
            if let Err(e) = stack.push(c) {
                print_error(e);
            }
        }
    }
}

pub fn execute_output_op(op: &OutputOperation, stack: &mut Stack) {
    match op {
        OutputOperation::Dot => match stack.pop() {
            Ok(a) => println!("{a}"),
            Err(e) => print_error(e),
        },
        OutputOperation::Cr => {
            println!();
        }
        OutputOperation::Emit => match stack.pop() {
            Ok(a) => {
                let ascii = a as u8;
                println!("{}", ascii as char);
            }
            Err(e) => print_error(e),
        },
        OutputOperation::DotQuote(text) => {
            println!("{text}");
        }
    }
}

pub fn execute_boolean_op(op: &BooleanOperation, stack: &mut Stack) {
    match op {
        BooleanOperation::Not => match stack.pop() {
            Ok(a) => {
                let result = if a != 0 { 0 } else { -1 };
                if let Err(e) = stack.push(result) {
                    print_error(e);
                }
            }
            Err(e) => print_error(e),
        },
        _ => {
            let a = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let b = match stack.pop() {
                Ok(val) => val,
                Err(e) => {
                    print_error(e);
                    return;
                }
            };
            let result = match op {
                BooleanOperation::Equal => a == b,
                BooleanOperation::Greater => a < b,
                BooleanOperation::Less => a > b,
                BooleanOperation::And => a == -1 && b == -1,
                BooleanOperation::Or => a == -1 || b == -1,
                _ => {
                    print_error(ForthError::Generic("Unknown boolean operation"));
                    false
                }
            };
            if let Err(e) = stack.push(if result { -1 } else { 0 }) {
                print_error(e);
            }
        }
    }
}

pub fn execute_conditional_op(
    op: &ConditionalOperation,
    stack: &mut Stack,
    execution_mode: &mut Vec<ExecutionStage>,
) {
    match op {
        ConditionalOperation::If => match stack.pop() {
            Ok(condition) => {
                if condition == 0 {
                    execution_mode.push(ExecutionStage::Skipping(1));
                } else {
                    execution_mode.push(ExecutionStage::Executing);
                }
            }
            Err(e) => print_error(e),
        },
        ConditionalOperation::Else => {
            if let Some(last) = execution_mode.last_mut() {
                match last {
                    ExecutionStage::Executing => {
                        *last = ExecutionStage::Skipping(1);
                    }
                    ExecutionStage::Skipping(depth) => {
                        if *depth == 1 {
                            if let Some(mode) = execution_mode.last_mut() {
                                *mode = ExecutionStage::Executing;
                            }
                        }
                    }
                }
            }
        }
        ConditionalOperation::Then => {
            if let Some(last) = execution_mode.last_mut() {
                match last {
                    ExecutionStage::Skipping(depth) => {
                        if *depth > 1 {
                            *depth -= 1;
                        } else {
                            execution_mode.pop();
                        }
                    }
                    _ => {
                        execution_mode.pop();
                    }
                }
            }
        }
    }
}

pub fn execute_other_operations(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match val {
        ForthValue::Operation(ForthOperation::Arithmetic(op)) => execute_arithmetic_op(op, stack),
        ForthValue::Operation(ForthOperation::StackTypeOp(op)) => execute_stack_op(op, stack),
        ForthValue::Operation(ForthOperation::Output(op)) => execute_output_op(op, stack),
        ForthValue::Operation(ForthOperation::Boolean(op)) => execute_boolean_op(op, stack),
        ForthValue::Number(n) => {
            if let Err(e) = stack.push(*n) {
                print_error(e);
            }
        }
        ForthValue::Word(ForthWord::WordStart(word_name)) => {
            if let Some(ref current) = current_word {
                if current == word_name {
                    return;
                }
            }

            if let Some(definition) = dictionary.get_word(word_name) {
                let mut execution_mode_stack = vec![ExecutionStage::Executing];
                for val in definition {
                    execute_instruction(
                        val,
                        stack,
                        dictionary,
                        &mut execution_mode_stack,
                        Some(word_name.to_string()),
                        executed_words,
                    );
                }
            } else {
                print_error(ForthError::UnknownWord);
            }
        }
        _ => {}
    }
}

pub fn execute_instruction(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    execution_mode: &mut Vec<ExecutionStage>,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match execution_mode.last().unwrap_or(&ExecutionStage::Executing) {
        ExecutionStage::Executing => match val {
            ForthValue::Word(ForthWord::WordStart(word_name)) => {
                if executed_words.contains(word_name) {
                    return;
                }

                executed_words.push(word_name.to_string());

                if let Some(definition) = dictionary.get_word(word_name) {
                    let mut mode_stack = vec![ExecutionStage::Executing];
                    for val in definition {
                        execute_instruction(
                            val,
                            stack,
                            dictionary,
                            &mut mode_stack,
                            Some(word_name.to_string()),
                            executed_words,
                        );
                    }
                } else {
                    print_error(ForthError::UnknownWord);
                }

                executed_words.pop();
            }
            ForthValue::Operation(ForthOperation::Conditional(op)) => {
                execute_conditional_op(op, stack, execution_mode);
            }
            _ => execute_other_operations(val, stack, dictionary, current_word, executed_words),
        },
        ExecutionStage::Skipping(_depth) => match val {
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Then)) => {
                if let Some(ExecutionStage::Skipping(current_depth)) = execution_mode.last_mut() {
                    if *current_depth > 1 {
                        *current_depth -= 1;
                    } else {
                        execution_mode.pop();
                    }
                }
            }
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::Else)) => {
                if let Some(ExecutionStage::Skipping(depth)) = execution_mode.last_mut() {
                    if *depth == 1 {
                        if let Some(last) = execution_mode.last_mut() {
                            *last = ExecutionStage::Executing;
                        }
                    }
                }
            }
            ForthValue::Operation(ForthOperation::Conditional(ConditionalOperation::If)) => {
                if let Some(ExecutionStage::Skipping(depth)) = execution_mode.last_mut() {
                    *depth += 1;
                }
            }
            _ => {}
        },
    }
}
