use crate::operations::{
    arithmetic::execute_arithmetic_op, boolean::execute_boolean_op, conditional::{ConditionalOperation, execute_conditional_op},
    forth_operation::ForthOperation, output::execute_output_op, stack_type::execute_stack_op,
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
        ForthValue::Word(ForthWord::Start(word_name)) => {
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
            ForthValue::Word(ForthWord::Start(word_name)) => {
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
