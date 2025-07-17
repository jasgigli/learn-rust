// This Rust program demonstrates integer types and arithmetic operations.
//
// To build the project (from 06-data-types):
//    cargo build
//
// To run a specific file (like this one) with Cargo:
//    cargo run --bin integer
//
// Integers are whole numbers. Rust supports signed and unsigned types.
fn main() {
    let x: i32 = 8;
    let y: i32 = 2;
    let add = x + y;
    let sub = x - y;
    let mul = x * y;
    let div = x / y;
    let rem = x % y;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("x + y = {add}");
    println!("x - y = {sub}");
    println!("x * y = {mul}");
    println!("x / y = {div}");
    println!("x % y = {rem}");
}
