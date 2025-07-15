[⬅️ Back to Main Table of Contents](../README.md)

# 📝 Variables & Mutability in Rust

## Overview

Variables are like labeled boxes where you store information. In Rust, these boxes are special: by default, once you put something in, you can’t change it! This helps keep your programs safe and bug-free.

But sometimes, you *do* want to change what’s inside. Rust lets you do that too—with a little extra care.

---

## Step 1: Immutable Variables (The Default)

Let’s start with a simple example:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // This line will cause an error!
    println!("The value of x is: {x}");
}
```

If you run this, Rust will give you an error. Why? Because `x` is *immutable*—you can’t change it after it’s set.

*Analogy:* Imagine writing with a permanent marker. Once you write something, you can’t erase or change it!

---

## Step 2: Making Variables Mutable

If you want to change the value, use `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // Now this works!
    println!("The value of x is: {x}");
}
```

Now, `x` is like a whiteboard—you can erase and write a new value.

---

## Step 3: Constants

Constants are like labels for values that never, ever change. You declare them with `const` and must always give them a type:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
- Use ALL_CAPS for constant names.
- Constants are always immutable and can be used anywhere in your code.

---

## Step 4: Shadowing

Rust lets you “shadow” a variable by declaring it again with the same name:

```rust
fn main() {
    let x = 5;
    let x = x + 1; // shadows the previous x
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6
}
```

*Analogy:* Shadowing is like putting a new label on your box. The old label is covered up, but the box is still there!

---

## Try It Yourself!

- Try making a variable mutable and changing its value.
- Declare a constant for the number of minutes in a day.
- Experiment with shadowing: can you change a variable’s type by shadowing?

---

## Next Steps

- You’ve learned how Rust keeps your data safe by default, but lets you change things when you need to.
- Move on to the next tutorial: Data Types
- Explore the [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

**Rust’s rules help you write safer, clearer code. Keep experimenting! 🦀**
