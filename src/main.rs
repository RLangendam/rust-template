//! CLI entry point with proper error handling

use std::process;

use my_project_lib::add;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Run the application.
///
/// # Errors
///
/// Returns an error if the operation fails.
#[allow(clippy::unnecessary_wraps)]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = add(10, 20);
    println!("Result: {result}");
    Ok(())
}
