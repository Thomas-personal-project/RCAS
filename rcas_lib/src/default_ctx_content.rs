use crate::context::Context;
use crate::debugger_pause;
use crate::token_defs::{Token, Variable};
use crate::{ctx, debug, end, fetch_pop, return_one_as};
use anyhow::Result;

/// + operator
fn add(tokens: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    let arg1 = fetch_pop!(tokens, Const);
    let arg2 = fetch_pop!(tokens, Const);
    let res = arg1 + arg2;

    return_one_as!(res, Const)
}
ctx!("+", add);

/// - operator
fn sub(tokens: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    let arg1 = fetch_pop!(tokens, Const);
    let arg2 = fetch_pop!(tokens, Const);
    let res = arg1 - arg2;

    return_one_as!(res, Const)
}
ctx!("-", sub);

/// * operator
fn mul(tokens: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    let arg1 = fetch_pop!(tokens, Const);
    let arg2 = fetch_pop!(tokens, Const);
    let res = arg1 * arg2;

    return_one_as!(res, Const)
}
ctx!("*", mul);

/// Print command
fn print(tokens: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    println!("{}", fetch_pop!(tokens));
    end!()
}
ctx!("Print", print);

/// Exit command
fn exit(_: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    std::process::exit(0)
}
ctx!("Exit", exit);

/// Nop (No operation) command
fn nop(_: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    end!()
}
ctx!("Nop", nop);

/// Clear command (clears stack)
fn clear(tokens: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    tokens.clear();
    end!();
}
ctx!("Clear", clear);

/// Assign (turns a string to a variable with a value)
fn assign(tokens: &mut Vec<Token>, ctx: &mut Context) -> Result<Vec<Token>> {
    let value = fetch_pop!(tokens);
    let name = fetch_pop!(tokens, String);

    ctx.variables.push(Variable {
        name,
        value: Some(Box::new(value)),
    });

    end!();
}
ctx!(":=", assign);

/// RevAssign (assignment but with backwards ordering)
fn rev_assign(tokens: &mut Vec<Token>, ctx: &mut Context) -> Result<Vec<Token>> {
    let name = fetch_pop!(tokens, String);
    let value = fetch_pop!(tokens);

    ctx.variables.push(Variable {
        name,
        value: Some(Box::new(value)),
    });

    end!();
}
ctx!("=:", rev_assign);

/// & operator (deletes a variable)
fn deassign(tokens: &mut Vec<Token>, ctx: &mut Context) -> Result<Vec<Token>> {
    let var = fetch_pop!(tokens, Variable);

    let idx = ctx.variables.iter().position(|x| *x == var).unwrap();
    ctx.variables.remove(idx);

    end!();
}
ctx!("&", deassign);

/// ::PAUSE operator (pauses execution for debugging)
#[cfg(feature = "debugger")]
fn debug_interrupt(_: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    debugger_pause();

    end!();
}
ctx!("::PAUSE", debug_interrupt);

/// ::STACK_JUMP (dumps the stack for debugging)
#[cfg(feature = "debugger")]
fn stack_dump(stack: &mut Vec<Token>, _: &mut Context) -> Result<Vec<Token>> {
    debug!("STACK_DUMP: {:?}", stack);
    end!();
}
ctx!("::STACK_DUMP", stack_dump);

/// ::CONTEXT_DUMP (dumps the context for debugging)
#[cfg(feature = "debugger")]
fn ctx_dump(_: &mut Vec<Token>, ctx: &mut Context) -> Result<Vec<Token>> {
    debug!("CONTEXT_DUMP: {:?}", ctx);
    end!();
}
ctx!("::CONTEXT_DUMP", ctx_dump);
