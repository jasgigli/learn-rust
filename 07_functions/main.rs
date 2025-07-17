// This Rust program demonstrates how to define and use functions.
//
// To build the project (from 07_functions):
//    cargo build
//
// To run the project:
//    cargo run
//
// Functions are declared with the fn keyword.
fn main() {
    greet();
    let sum = add(5, 3);
    println!("The sum of 5 and 3 is: {sum}");
}

// A simple function that prints a greeting
fn greet() {
    println!("Hello from a function!");
}

// A function that takes two parameters and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}
