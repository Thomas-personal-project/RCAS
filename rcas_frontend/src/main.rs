use rcas_lib::{
    context::{Context, get_default_functions},
    // debugger::{BufExecDebugger, IntoDebugger, Stepper},
    parse_rpol_notation::*,
    stack_machine::{BufferedExecutor, RevPolStackMachine},
};

fn main() {
    let file = "D:\\rcas\\test_files\\test_file.mir";
    println!("{}", file);

    let file_content = get_content_at(file.to_string()).unwrap();

    let commands: Vec<Vec<String>> = split_into_commands(file_content);

    let mut context = Context::new();
    context.functions.append(&mut get_default_functions());

    let exec_cmds = commands_to_sequential_exec_order(commands, &context);

    println!("Now running!");

    let stack_machine = RevPolStackMachine::new_with_ctx(context);
    let mut stack_executor = BufferedExecutor::new(stack_machine, exec_cmds.unwrap());
    stack_executor.run_stack().unwrap();

    // let debug = Stepper::new(BufExecDebugger::new(stack_executor));
    //
    // println!("Initial snapshot:\n{}", debug.inner.static_snapshot());
    // println!("------------------------------------------------");
    //
    // for frame in debug.into_iter() {
    //     println!("{}", frame);
    //     println!("------------------------------------------------");
    // }
}
