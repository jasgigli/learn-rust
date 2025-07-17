// This Rust program demonstrates variables, mutability, and constants.
//
// In this folder (05-variables-mutability), we use Cargo to build and run the project.
//
// To build the project:
//    cargo build
//
// To run the project:
//    cargo run
//
// Step 1: Immutable variable (default in Rust)
// let x = 5; // x cannot be changed
//
// Step 2: Mutable variable
// let mut y = 10; // y can be changed
//
// Step 3: Constant
// const MAX_POINTS: u32 = 100_000; // Always immutable, type required

fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // Uncommenting this line will cause a compile error

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {y}");
    y = 15; // This is allowed because y is mutable
    println!("The new value of y is: {y}");

    // Constant
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("There are {SECONDS_IN_MINUTE} seconds in a minute.");

    // Shadowing
    let z = 7;
    let z = z + 1; // shadows previous z
    println!("The value of shadowed z is: {z}");
}
