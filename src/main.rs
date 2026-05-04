//! Use the library logic; main.rs should be a thin wrapper.
use my_project_lib::add;

fn main() {
    let result = add(10, 20);
    println!("Result: {result}");
}
