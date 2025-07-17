# Rust Comments

This section demonstrates how to write comments in Rust.

---

## Example: Writing Comments

See [`main.rs`](main.rs) for a complete example.

### Types of Comments
- **Single-line comments:** Start with `//` and continue to the end of the line.
- **Multi-line comments:** Start with `/*` and end with `*/`. Can span multiple lines.

### Example Usage
```rust
// This is a single-line comment.

/*
   This is a multi-line comment.
   Everything between the /* and */ is ignored by the compiler.
*/

fn main() {
    // Print a message to the console
    println!("Comments make your code easier to understand!");
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

For more details, see the [Rust Book: Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
