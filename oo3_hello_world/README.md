[⬅️ Back to Main Table of Contents](../README.md)

# 👋 Your First Rust Program: Hello, World!

## Overview

Let’s write your very first Rust program! This is like saying “hello” to a new friend. In programming, we start with a simple message: `Hello, world!`

You don’t need any experience—just follow along and you’ll see your computer talk back to you.

---

## Step 1: Set Up a Folder

Let’s keep things tidy by making a folder for your code.

- **Windows:**
  1. Open Command Prompt or PowerShell.
  2. Type:
     ```powershell
     mkdir hello_world
     cd hello_world
     ```
- **Mac/Linux:**
  1. Open Terminal.
  2. Type:
     ```sh
     mkdir hello_world
     cd hello_world
     ```

---

## Step 2: Write the Code

Create a file called `main.rs` in your new folder. Open it in your favorite editor, then copy and paste this:

```rust
fn main() {
    println!("Hello, world!");
}
```

**What does this mean?**
- `fn main() { ... }` — This is the “main” function. It’s where your program starts, like the front door to your house.
- `println!(...)` — This tells Rust to print something on the screen. The `!` means it’s a macro (a special kind of command).
- `"Hello, world!"` — The message you want to show.
- The semicolon `;` ends the instruction, like a period in a sentence.

*Analogy:* Writing a program is like writing a recipe. The main function is the title, and each line inside is a step.

---

## Step 3: Run Your Program

Let’s see your code in action!

- **Windows (PowerShell or CMD):**
  ```powershell
  rustc main.rs
  .\main
  ```
- **Mac/Linux:**
  ```sh
  rustc main.rs
  ./main
  ```

You should see:
```
Hello, world!
```

**What just happened?**
- `rustc main.rs` — This compiles (translates) your code into a program your computer can run.
- `./main` or `.\main` — This runs your new program.

---

## Step 4: Try It Yourself!

- Change the message inside the quotes. What happens?
- Add another line: `println!("Rust is fun!");`
- Try removing the semicolon—what error do you get?

---

## Next Steps

- Celebrate! You just wrote and ran your first Rust program. 🎉
- Move on to the next tutorial: [Hello Cargo](../oo4_hello_cargo/README.md)
- Explore the [Rust Playground](https://play.rust-lang.org/) to experiment online.

---

**You’re officially a Rustacean. Welcome! 🦀**
