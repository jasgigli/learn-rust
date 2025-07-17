<!--
Meta Description: Beginner's guide to data types in Rust. Learn about numbers, booleans, characters, tuples, and arrays in Rust. Step-by-step Rust data types tutorial for new Rustaceans.
-->
[⬅️ Back to Main Page](../README.md)

---

# Rust Data Types

This section covers Rust's data types, including scalar and compound types, with code examples and instructions for running them using Cargo.

---

## Scalar Types

Scalar types represent a single value. Rust has four primary scalar types:

### 1. Integer
- Example: [`01_scalar_type/integer.rs`](01_scalar_type/integer.rs)
- **Description:** Whole numbers, both signed and unsigned, e.g., `i32`, `u8`.
- **Run with Cargo:**
  ```sh
  cargo run --bin integer
  ```

### 2. Floating Point
- Example: [`01_scalar_type/floating_point.rs`](01_scalar_type/floating_point.rs)
- **Description:** Numbers with decimal points, e.g., `f32`, `f64`.
- **Run with Cargo:**
  ```sh
  cargo run --bin floating_point
  ```

### 3. Boolean
- Example: [`01_scalar_type/boolean.rs`](01_scalar_type/boolean.rs)
- **Description:** `true` or `false` values.
- **Run with Cargo:**
  ```sh
  cargo run --bin boolean
  ```

### 4. Character
- Example: [`01_scalar_type/character.rs`](01_scalar_type/character.rs)
- **Description:** Single Unicode scalar values, e.g., `'a'`, `'∞'`.
- **Run with Cargo:**
  ```sh
  cargo run --bin character
  ```

---

## Compound Types

Compound types can group multiple values into one type.

### 1. Tuple
- Example: [`02_compound_types/main.rs`](02_compound_types/main.rs)
- **Description:** Group values of different types together.
- **Run with Cargo:**
  ```sh
  cargo run --bin main
  ```

---

## How to Build All Examples

From this directory, you can build all examples with:
```sh
cargo build
```

To run a specific example, use the `--bin` flag as shown above.

---

For more details, see the [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
