use crate::context::Context;
use anyhow::Result;
use std::boxed::Box;
use std::fmt::Display;
use crate::number::Number;

/// fn(&mut stack) -> push_to_stack
pub type FunctionObject = fn(&mut Vec<Token>, &mut Context) -> Result<Vec<Token>>;

/// An abstraction over the name and possible value of a variable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub value: Option<Box<Token>>,
}
impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(token) => write!(f, "{} = {}", self.name, token),
            None => write!(f, "{} (undefined)", self.name),
        }
    }
}

/// Represents some arbitrary rust function imported in under a new name, for example the Exit
/// function will ignore the stack and just quit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Functor {
    pub name: String,
    pub func: FunctionObject,
}
impl Display for Functor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(x) -> {:?}", self.name, self.func)
    }
}

/// Represents what a token could be. Everything is either a Constant, Variable, Functor or special
/// character or delimeter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Const(Number),
    Variable(Variable),
    Functor(Functor),
    String(String),
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Const(num) => write!(f, "{}", num),
            Token::Variable(var) => write!(f, "{}", var),
            Token::Functor(func) => write!(f, "{}", func),
            Token::String(str) => write!(f, "{}", str),
        }
    }
}
