// This Rust program demonstrates loops in Rust.
//
// To build the project (from 09_control_flow/02_loops):
//    cargo build
//
// To run the project:
//    cargo run
//
fn main() {
    // Infinite loop with break
    let mut count = 0;
    loop {
        if count == 3 {
            break;
        }
        println!("loop count: {count}");
        count += 1;
    }

    // While loop
    let mut n = 0;
    while n < 3 {
        println!("while n: {n}");
        n += 1;
    }

    // For loop
    for i in 0..3 {
        println!("for i: {i}");
    }
}
