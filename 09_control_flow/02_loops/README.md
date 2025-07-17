# Rust Control Flow: Loops

This section demonstrates how to use loops in Rust.

---

## Example: Loops

See [`main.rs`](main.rs) for a complete example.

### Types of Loops
- **loop:** Repeats forever unless you explicitly break.
- **while:** Repeats while a condition is true.
- **for:** Iterates over a range or collection.

### Example Usage
```rust
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
```

---

## How to Build and Run

From this directory:
```sh
cargo build
cargo run
```

---

For more details, see the [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
