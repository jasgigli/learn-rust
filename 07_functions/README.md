# Rust Functions

This section demonstrates how to define and use functions in Rust.

---

## Example: Defining and Using Functions

See [`main.rs`](main.rs) for a complete example.

### Key Points
- Functions are declared with the `fn` keyword.
- Parameters and return types are specified in the function signature.
- Functions help organize and reuse code.

### Example Usage
```rust
fn main() {
    greet();
    let sum = add(5, 3);
    println!("The sum of 5 and 3 is: {sum}");
}

fn greet() {
    println!("Hello from a function!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## How to Build and Run

From this directory:
```sh
cargo build
cargo run
```

---

For more details, see the [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
