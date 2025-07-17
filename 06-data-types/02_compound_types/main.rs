// This Rust program demonstrates compound types: tuples.
//
// To build the project (from 06-data-types):
//    cargo build
//
// To run the project:
//    cargo run --bin main
//
// Tuples group values of different types into a single compound type.
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // Accessing tuple elements directly
    println!("First value: {}", tup.0);
    println!("Second value: {}", tup.1);
    println!("Third value: {}", tup.2);
}
