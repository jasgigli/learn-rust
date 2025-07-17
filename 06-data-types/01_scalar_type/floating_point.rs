// This Rust program demonstrates floating point types (f32, f64).
//
// To build the project (from 06-data-types):
//    cargo build
//
// To run a specific file (like this one) with Cargo:
//    cargo run --bin floating_point
//
// Floating point numbers are numbers with decimals.
fn main() {
    let num_one: f64 = 3.14;
    let num_two: f64 = 6.8;
    let sum = num_one + num_two;
    println!("num_one + num_two = {sum}");
}
