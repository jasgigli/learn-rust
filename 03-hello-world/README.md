<!--
Meta Description: Step-by-step Rust Hello World tutorial for beginners. Learn how to write and run your first Hello World program in Rust. Includes code, explanations, and tips for new Rustaceans.
-->
[⬅️ Back to Main Table of Contents](../README.md)

# 👋 Rust Hello World Tutorial: Your First Program

## What is "Hello World" in Rust?

The "Hello World" program is the classic first step in learning any programming language. In this Rust beginner tutorial, you'll learn how to write, run, and understand your first Rust program.

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

## Step 2: Write the Rust Hello World Code

Create a file called `main.rs` in your new folder. Open it in your favorite editor, then copy and paste this:

```rust
fn main() {
    println!("Hello, world!");
}
```

**What does this mean?**
- `fn main() { ... }` — This is the main function. Every Rust program starts here.
- `println!(...)` — This prints text to the screen. The `!` means it’s a macro.
- `"Hello, world!"` — The message you want to show.
- The semicolon `;` ends the instruction.

*Tip:* Writing a program is like writing a recipe. The main function is the title, and each line inside is a step.

---

## Step 3: Run Your Rust Program

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
- `rustc main.rs` — Compiles your code into a program your computer can run.
- `./main` or `.\main` — Runs your new program.

---

## Step 4: Experiment and Learn

- Change the message inside the quotes. What happens?
- Add another line: `println!("Rust is fun!");`
- Try removing the semicolon—what error do you get?

---

## Next Steps for Rust Beginners

- 🎉 You just wrote and ran your first Rust program!
- Move on to the next tutorial: [Rust Cargo Tutorial](../04-cargo/README.md)
- Explore the [Rust Playground](https://play.rust-lang.org/) to experiment online.
- Read the [Official Rust Book: Hello World](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

---

**You’re officially a Rustacean. Welcome! 🦀**
