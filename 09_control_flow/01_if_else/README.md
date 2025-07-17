# Rust Control Flow: if-else

This section demonstrates how to use if-else statements in Rust.

---

## Example: if-else

See [`main.rs`](main.rs) for a complete example.

### Key Points
- Use `if`, `else if`, and `else` to control program flow based on conditions.
- Conditions must be boolean expressions.

### Example Usage
```rust
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
