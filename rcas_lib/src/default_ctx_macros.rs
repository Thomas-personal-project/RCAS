use crate::context::Context;
use crate::token_defs::Functor;
use lazy_static::lazy_static;
use linkme::distributed_slice;

#[distributed_slice]
pub static DEFAULT_FUNCTIONS: [(&'static str, crate::token_defs::FunctionObject)] = [..];

#[macro_export]
macro_rules! ctx {
    ($name:expr, $func:ident) => {
        paste::paste! {
            #[::linkme::distributed_slice(crate::default_ctx_macros::DEFAULT_FUNCTIONS)]
            #[allow(non_upper_case_globals)]
            static [<_REGISTER_ $func>]: (
                &'static str,
                crate::token_defs::FunctionObject
            ) = ($name, $func);
        }
    };
}

lazy_static! {
    pub static ref DEFAULT_CTX: Context = {
        let mut ctx = Context::new();
        for (name, func) in DEFAULT_FUNCTIONS.iter() {
            ctx.functions.push(Functor {
                name: name.to_string(),
                func: *func,
            });
        }
        ctx
    };
}

/// Fetch_pop is a syntactic sugar macro
/// Branch 1 matches fetch_pop!(tokens), where the first argument must be your argument tokens. It
/// returns an argument from the tokens or panics, saving the check yourself.
/// Branch 2 maches fetch_pop!(tokens, Variant), where the first argument must be your argument
/// tokens, and the second is any variant of the Tokens enum. It returns the top argument from
/// the tokens which is variant Variant, returning the inner value of that variant, again saving
/// this check yourself.
#[macro_export]
macro_rules! fetch_pop {
    ($tokens:ident) => {
        match $tokens.pop() {
            Some(t) => t,
            _ => anyhow::bail!("Not enough arguments"),
        }
    };

    ($tokens:ident, $variant:ident) => {
        match $tokens.pop() {
            Some(crate::token_defs::Token::$variant(item)) => item,
            _ => anyhow::bail!("Incorrect argument type or number"),
        }
    };
}

/// Takes in a single item and it's associated token variant, and returns it in proper form
#[macro_export]
macro_rules! return_one_as {
    ($input:ident, $variant:ident) => {
        return Ok(vec![crate::token_defs::Token::$variant($input)])
    };
}

#[macro_export]
macro_rules! return_one {
    ($input: ident) => {
        return Ok(vec![$input])
    };
}

/// Returns no tokens
#[macro_export]
macro_rules! end {
    () => {
        return Ok(vec![])
    };
}
