// This Rust program demonstrates if-else control flow.
//
// To build the project (from 09_control_flow/01_if_else):
//    cargo build
//
// To run the project:
//    cargo run
//
fn main() {
    let number = 7;
    if number < 5 {
        println!("{number} is less than 5");
    } else if number == 5 {
        println!("{number} is equal to 5");
    } else {
        println!("{number} is greater than 5");
    }
}
