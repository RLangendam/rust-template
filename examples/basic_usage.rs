//! # Basic Usage Example
//!
//! This example demonstrates how to use the `my_project_lib` library
//! to perform simple arithmetic operations.

use my_project_lib::add;

/// Demonstrate const function usage at compile time
const COMPILE_TIME_RESULT: usize = add(100, 200);

fn main() {
    println!("🦀 Rust Template Example");
    println!("========================");

    // Basic addition
    let result1 = add(10, 20);
    println!("10 + 20 = {result1}");

    // More examples
    let result2 = add(42, 58);
    println!("42 + 58 = {result2}");

    // Use the compile-time computed result
    println!("Compile-time result: {COMPILE_TIME_RESULT}");

    println!("\n✅ Example completed successfully!");
}
