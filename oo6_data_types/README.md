# 🔢 Data Types in Rust

## Overview

Every value in Rust has a type—like how every item in your kitchen has a purpose (spoon, bowl, pan). Types help Rust keep your code safe and fast. Let’s explore the most common types you’ll use!

---

## Step 1: Numbers

Rust has two main kinds of numbers:
- **Integers:** Whole numbers (like 1, -42, 100)
- **Floats:** Numbers with decimals (like 3.14, -0.5)

Example:
```rust
fn main() {
    let apples: i32 = 5;      // integer
    let price: f64 = 2.99;    // floating-point
    println!("I have {apples} apples, each costs ${price}");
}
```
- `i32` means a 32-bit integer (most common)
- `f64` means a 64-bit floating-point number (most common for decimals)

*Analogy:* Integers are like whole apples, floats are like slices!

---

## Step 2: Booleans & Characters

- **Booleans:** true or false
- **Characters:** Single letters, numbers, or symbols (in single quotes)

Example:
```rust
fn main() {
    let is_rust_fun: bool = true;
    let letter: char = 'R';
    println!("Is Rust fun? {is_rust_fun} — My favorite letter: {letter}");
}
```

---

## Step 3: Tuples & Arrays

- **Tuples:** Group different types together
- **Arrays:** List of values, all the same type, fixed size

Example:
```rust
fn main() {
    let person: (i32, f64, char) = (25, 72.5, 'A');
    let scores: [i32; 3] = [90, 85, 88];
    println!("Person: age {}, weight {}, grade {}", person.0, person.1, person.2);
    println!("Scores: {} {} {}", scores[0], scores[1], scores[2]);
}
```
- Access tuple items with `.0`, `.1`, etc.
- Access array items with `[index]`

*Analogy:* A tuple is like a lunchbox with a sandwich, apple, and juice. An array is like a carton of six eggs—all the same!

---

## Try It Yourself!
- Make a variable for your age (integer), your height (float), and your first initial (char).
- Create an array of your three favorite numbers.
- Make a tuple with your name (as a char), age, and a boolean for “likes Rust.”

---

## Next Steps
- You’ve learned the building blocks of Rust data!
- Try changing types and see what errors you get.
- Move on to the next topic, or explore the [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

---

**Types help Rust help you. Keep experimenting! 🦀**
