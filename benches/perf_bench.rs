//! A simple benchmark example
use my_project_lib::add;
use std::hint::black_box;

fn main() {
    // Basic manual bench loop
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        black_box(add(2, 2));
    }
    println!("Time: {:?}", start.elapsed());
}
