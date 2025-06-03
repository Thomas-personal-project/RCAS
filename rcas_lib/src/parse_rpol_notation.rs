use anyhow::{Result, bail};
use num_bigfloat::BigFloat;
use num_bigint::BigInt;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

use crate::context::Context;
use crate::token_defs::{Token};
use crate::number::Number;

/// Just a simple function to take the content at a file
pub fn get_content_at(file: String) -> Result<String> {
    let path = Path::new(&file);

    if !path.exists() {
        bail!("Failed to find file <{}>", file)
    }

    let content = read_to_string(path)?;

    return Ok(content.trim().replace("\r", "").to_string());
}

/// A simple function to take content of a file and split it up into a sequential list of commands
/// e.g. 1.2 5 + -> [Token::Const(Number::float(1.2)), Token::Const(Number::Int(5)),
/// Token::Function(5)]
pub fn split_into_commands(input: String) -> Vec<Vec<String>> {
    let input_lines = input.split('\n');
    let mut next_split = vec![];

    // Helper closure to split a line into tokens while respecting quoted strings
    let split_line = |line: &str| -> Vec<String> {
        let mut tokens = Vec::new();
        let mut start = 0; // Byte index for the start of the current token
        let mut inside_quotes = false;

        // Iterate over each character with its byte index
        for (i, c) in line.char_indices() {
            if c == '"' {
                // Toggle quote state when encountering a double quote
                inside_quotes = !inside_quotes;
            } else if c == ' ' && !inside_quotes {
                // Split at space only when not inside quotes
                tokens.push(line[start..i].to_string());
                start = i + 1; // Move start past the space
            }
        }
        // Push the last token after processing all characters
        tokens.push(line[start..].to_string());
        tokens
    };

    for item in input_lines {
        // Skip comment lines (starting with "//")
        if item.starts_with("//") {
            continue;
        }
        // Process the line and add its tokens to the result
        next_split.push(split_line(item));
    }

    next_split
}

/// Maps over commands to create tokens
pub fn commands_to_sequential_exec_order(
    input: Vec<Vec<String>>,
    context: &Context,
) -> Result<Vec<Vec<Token>>> {
    input
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|string| single_command_to_token(string, context))
                .collect()
        })
        .collect()
}

/// Turn a single string into a RevPol token
pub fn single_command_to_token(input: String, context: &Context) -> Result<Token> {
    // Variable | String
    if input.starts_with("\"") && input.ends_with("\"") {
        let string_content = input[1..input.len() - 1].to_string();

        for item in context.variables.iter() {
            if item.name == string_content {
                return Ok(Token::Variable(item.clone()));
            }
        }

        return Ok(Token::String(string_content));
    }

    // Constant
    if let Ok(int) = BigInt::from_str(&input) {
        return Ok(Token::Const(Number::Int(int)));
    } else if let Ok(float) = BigFloat::from_str(&input) {
        return Ok(Token::Const(Number::Float(float)));
    }

    // Variable
    if let Some(var) = context.variables.iter().find_map(|variable| {
        if variable.name == input {
            return Some(variable);
        }

        return None;
    }) {
        return Ok(Token::Variable(var.clone()));
    }

    // Function
    if let Some(func) = context.functions.iter().find_map(|function| {
        if function.name == input {
            return Some(function);
        }

        return None;
    }) {
        return Ok(Token::Functor(func.clone()));
    }

    // Nothing?? Invalid!
    bail!("Unqualified token found: {}", input)
}
