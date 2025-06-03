use crate::default_ctx_macros::DEFAULT_FUNCTIONS;
use crate::token_defs::{Functor, Variable};

/// A Context is a vec of variables and functions which have been provided already. This is
/// used to tell the stack machine what some functions and variables are, for example passing pi or
/// e through as constants.
#[derive(Debug, Clone)]
pub struct Context {
    pub variables: Vec<Variable>,
    pub functions: Vec<Functor>,
}
impl Context {
    pub fn new() -> Self {
        return Context {
            variables: vec![],
            functions: vec![],
        };
    }
}

/// Grabs the default set of functions
pub fn get_default_functions() -> Vec<Functor> {
    DEFAULT_FUNCTIONS
        .to_vec()
        .iter()
        .map(|f| Functor {
            name: f.0.to_string(),
            func: f.1,
        })
        .collect::<Vec<Functor>>()
}
