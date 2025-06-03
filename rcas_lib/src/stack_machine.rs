use crate::context::Context;
use crate::debug;
use crate::token_defs::Token;
use anyhow::{Result, bail};

#[derive(Debug)]
pub struct RevPolStackMachine {
    pub stack: Vec<Token>,
    pub context: Context,
}
impl RevPolStackMachine {
    pub fn new() -> Self {
        return RevPolStackMachine {
            stack: vec![],
            context: Context::new(),
        };
    }

    pub fn new_with_ctx(context: Context) -> Self {
        return RevPolStackMachine {
            stack: vec![],
            context,
        };
    }
}

impl Iterator for RevPolStackMachine {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check what's at the top of the stack
        match self.stack.pop()? {
            Token::Const(const_val) => {
                debug!("Popped Token::Const: {}", const_val);
                return Some(Ok(Token::Const(const_val)));
            }
            Token::Variable(var) => {
                debug!("Popped Token::Variable: {}", var);
                return Some(Ok(Token::Variable(var)));
            }
            Token::String(str) => {
                debug!("Popped Token::String: {}", str);
                return Some(Ok(Token::String(str)));
            }
            Token::Functor(func) => {
                // Behaviour on popping a functor is to feed it the stack as an argument
                // and feed the output to the stack again
                // NOTE: This is where the functions are actually executed
                debug!("Popped Token::Functor: {}", func.name);
                let res = (func.func)(&mut self.stack, &mut self.context);
                debug!("|-> Result: {:?}", res);

                let mut result = match res {
                    Err(e) => return Some(Err(e)),
                    Ok(result) => result,
                };

                self.stack.append(&mut result);
                None
            }
        }
    }
}

fn line_is_done(line: &Vec<Token>) -> bool {
    !line.iter().any(|token| matches!(token, Token::Functor(_)))
}

/// Executes a file of .mir - executes each line, until no more functions are present, then appends
/// the next line onto that, allowing for line breaks to execute functions on the results of others
pub struct BufferedExecutor {
    pub machine: RevPolStackMachine,
    pub lines: Vec<Vec<Token>>,
    pub current_line: usize,
}
impl BufferedExecutor {
    pub fn new(machine: RevPolStackMachine, lines: Vec<Vec<Token>>) -> Self {
        BufferedExecutor {
            machine,
            lines,
            current_line: 0,
        }
    }

    /// Helper function: checks if the executor is 'done' (the current_line = lines.len())
    pub(crate) fn machine_is_complete(&self) -> bool {
        let cond = self.current_line == self.lines.len();
        cond
    }

    fn preprocess_str(&mut self, tokens: &mut Vec<Token>) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        for token in tokens.iter() {
            let rectx_token = match token {
                Token::String(str) => {
                    if let Some(var) = self
                        .machine
                        .context
                        .variables
                        .iter()
                        .find(|p| p.name == *str)
                    {
                        // Clone the variable if needed, or use reference if your Token supports it
                        Token::Variable(var.clone())
                    } else {
                        Token::String(str.clone())
                    }
                }
                // Clone other tokens instead of returning
                other => other.clone(),
            };
            res.push(rectx_token);
        }

        res
    }

    /// Helper function: populates the machine with the current line and then runs it until it
    /// completes, then increments the current line count.
    pub fn run_line(&mut self) -> Result<()> {
        let mut line = self.lines[self.current_line].clone();
        let mut line = self.preprocess_str(&mut line);

        self.machine.stack.append(&mut line);

        while !line_is_done(&self.machine.stack) {
            let val = self.machine.next();

            if let Some(Err(_)) = val {
                bail!(
                    "Failed to execute BufferedReader line: {}",
                    self.current_line
                )
            }
        }

        self.current_line += 1;
        Ok(())
    }

    /// Run the entire stack that has been created here!
    pub fn run_stack(&mut self) -> Result<()> {
        while !self.machine_is_complete() {
            self.run_line()?
        }

        Ok(())
    }
}
