pub mod context;
pub mod default_ctx_content;
pub mod default_ctx_macros;
pub mod parse_rpol_notation;
pub mod stack_machine;
pub mod token_defs;
pub mod number;

#[cfg(feature = "debugger")]
pub mod debugger;

#[cfg(feature = "debugger")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        println!("[DEBUG] {}:{} - {}", file!(), line!(), format_args!($($arg)*));
    }};
}

/// Pauses execution until user presses Enter
#[cfg(feature = "debugger")]
pub fn debugger_pause() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    debug!("Press Enter to continue...");

    handle
        .read_line(&mut String::new())
        .expect("Failed to read line");

    print!("\x1B[2K\x1B[1G"); // Clears line and moves cursor to start
}

#[cfg(not(feature = "debugger"))]
pub fn debugger_pause() {}

#[cfg(not(feature = "debugger"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}
