#![allow(missing_docs)]
use crate::{
    context::Context,
    stack_machine::{BufferedExecutor, RevPolStackMachine},
    token_defs::Token,
};
use anyhow::Result;

pub struct RevPolBufSnapshot {
    pub stack_machine_stack: Vec<Token>,
    pub stack_machine_ctx: Context,
    pub lines: Vec<Vec<Token>>,
    pub current_line: usize,
    pub machine_is_complete: bool,
    pub last_result: Option<Result<()>>,
}
impl RevPolBufSnapshot {
    pub fn from_machine(machine: &BufferedExecutor, lines: Vec<Vec<Token>>) -> RevPolBufSnapshot {
        Self {
            stack_machine_stack: machine.machine.stack.clone(),
            stack_machine_ctx: machine.machine.context.clone(),
            lines,
            current_line: machine.current_line,
            machine_is_complete: machine.machine_is_complete(),
            last_result: None,
        }
    }

    pub fn from_machine_with_result(
        machine: &BufferedExecutor,
        lines: Vec<Vec<Token>>,
        result: Result<()>,
    ) -> RevPolBufSnapshot {
        Self {
            stack_machine_stack: machine.machine.stack.clone(),
            stack_machine_ctx: machine.machine.context.clone(),
            lines,
            current_line: machine.current_line,
            machine_is_complete: machine.machine_is_complete(),
            last_result: Some(result),
        }
    }

    pub fn is_error(&self) -> bool {
        if let Some(Err(_)) = self.last_result {
            return true;
        }

        return false;
    }
}
impl std::fmt::Display for RevPolBufSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack: {:#?}", self.stack_machine_stack)?;
        write!(f, "\nContext: {:#?}", self.stack_machine_ctx)?;
        write!(f, "\nLines: {:#?}", self.lines)?;
        write!(f, "\nCurrent line: {}", self.current_line)?;
        write!(f, "\nMachine is done: {}", self.machine_is_complete)?;
        write!(f, "\nLast result: {:?}", self.last_result)?;

        Ok(())
    }
}

pub trait IntoDebugger<Snapshot> {
    fn next_with_snapshot(&mut self) -> Snapshot;
    fn static_snapshot(&self) -> Snapshot;
    fn is_done(&self) -> bool;
}

pub struct BufExecDebugger(BufferedExecutor);
impl BufExecDebugger {
    pub fn new(executor: BufferedExecutor) -> Self {
        Self(executor)
    }

    pub fn new_machine(inner: RevPolStackMachine, lines: Vec<Vec<Token>>) -> Self {
        Self(BufferedExecutor::new(inner, lines))
    }

    pub fn machine_is_complete(&self) -> bool {
        self.0.machine_is_complete()
    }

    pub fn run_line(&mut self) -> Result<()> {
        self.0.run_line()
    }
}
impl IntoDebugger<RevPolBufSnapshot> for BufExecDebugger {
    fn is_done(&self) -> bool {
        self.machine_is_complete()
    }
    fn next_with_snapshot(&mut self) -> RevPolBufSnapshot {
        let result = self.run_line();
        let mut snap = self.static_snapshot();
        snap.last_result = Some(result);
        snap
    }
    fn static_snapshot(&self) -> RevPolBufSnapshot {
        RevPolBufSnapshot::from_machine(&self.0, self.0.lines.clone())
    }
}

pub struct Stepper<W, T: IntoDebugger<W>> {
    pub inner: T,
    phantom: std::marker::PhantomData<W>,
}
impl<W, T: IntoDebugger<W>> Stepper<W, T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            phantom: std::marker::PhantomData
        }
    }
}
impl<W, T: IntoDebugger<W>> Iterator for Stepper<W, T> {
    type Item = W;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_done() {
            return None;
        }

        Some(self.inner.next_with_snapshot())
    }
}
