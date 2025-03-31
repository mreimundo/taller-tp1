use crate::operations::{
    arithmetic::execute_arithmetic_op,
    boolean::execute_boolean_op,
    conditional::{ConditionalOperation, execute_conditional_op},
    forth_operation::ForthOperation,
    output::execute_output_op,
    stack_type::execute_stack_op,
};
use crate::{
    errors::{ForthError, print_error},
    forth_value::ForthValue,
    stack::Stack,
    words::{
        dictionary::WordsDictionary,
        word::{ForthWord, handle_word_execution},
    },
};

/// Enum that represents the stage of the execution (mainly used in ifs and words to control).
///
/// The different ones are:
///
/// - Executing: the instruction is executing.
/// - Skipping: tuple that contains the number of times (usually representing the depth) that the stage is skipped.
///

#[derive(PartialEq)]
pub enum ExecutionStage {
    Executing,
    Skipping(usize),
}

/// Execute different operations depending on the ForthValue reference "val" received by parameter. It receives the stack aswell to pass to the different execute_operations to update it.
/// The function also receives a reference to WordsDictionary to storage words, the current word that is executing, and a vector of the executed words to pass to execute_instruction.

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
                let mut execution_stage_stack = vec![ExecutionStage::Executing];
                for val in definition {
                    execute_instruction(
                        val,
                        stack,
                        dictionary,
                        &mut execution_stage_stack,
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

/// Execute different instructions depending on the ForthValue reference "val" received by parameter. It receives the stack aswell to pass to handle_executing_mode.
/// The function also receives a reference to WordsDictionary to storage words, the current word that is executing, and a vector of the executed words to pass to handle_executing_mode.

pub fn execute_instruction(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    execution_stage: &mut Vec<ExecutionStage>,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match execution_stage.last().unwrap_or(&ExecutionStage::Executing) {
        ExecutionStage::Executing => handle_executing_mode(
            val,
            stack,
            dictionary,
            execution_stage,
            current_word,
            executed_words,
        ),
        ExecutionStage::Skipping(_) => handle_skipping_mode(val, execution_stage),
    }
}

fn handle_executing_mode(
    val: &ForthValue,
    stack: &mut Stack,
    dictionary: &WordsDictionary,
    execution_stage: &mut Vec<ExecutionStage>,
    current_word: Option<String>,
    executed_words: &mut Vec<String>,
) {
    match val {
        ForthValue::Word(ForthWord::Start(word_name)) => {
            handle_word_execution(&word_name.to_string(), stack, dictionary, executed_words);
        }
        ForthValue::Operation(ForthOperation::Conditional(op)) => {
            execute_conditional_op(op, stack, execution_stage);
        }
        _ => execute_other_operations(val, stack, dictionary, current_word, executed_words),
    }
}

fn handle_skipping_mode(val: &ForthValue, execution_stage: &mut Vec<ExecutionStage>) {
    if let ForthValue::Operation(ForthOperation::Conditional(op)) = val {
        match op {
            ConditionalOperation::Then => handle_skipping_then(execution_stage),
            ConditionalOperation::Else => handle_skipping_else(execution_stage),
            ConditionalOperation::If => handle_skipping_if(execution_stage),
        }
    }
}

fn handle_skipping_then(execution_stage: &mut Vec<ExecutionStage>) {
    if let Some(ExecutionStage::Skipping(current_depth)) = execution_stage.last_mut() {
        if *current_depth > 1 {
            *current_depth -= 1;
        } else {
            execution_stage.pop();
        }
    }
}

fn handle_skipping_else(execution_stage: &mut [ExecutionStage]) {
    if let Some(ExecutionStage::Skipping(depth)) = execution_stage.last_mut() {
        if *depth == 1 {
            if let Some(last) = execution_stage.last_mut() {
                *last = ExecutionStage::Executing;
            }
        }
    }
}

fn handle_skipping_if(execution_stage: &mut [ExecutionStage]) {
    if let Some(ExecutionStage::Skipping(depth)) = execution_stage.last_mut() {
        *depth += 1;
    }
}
