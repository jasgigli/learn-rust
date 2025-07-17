<!--
Meta Description: Beginner's guide to Cargo, Rust's project manager. Learn how to create, build, and run Rust projects with Cargo. Step-by-step Rust Cargo tutorial for new Rustaceans.
-->
[⬅️ Back to Main Page](../README.md)

---

# 📦 Rust Cargo Tutorial: Project Management for Beginners

## What is Cargo in Rust?

Cargo is Rust’s official project manager and build tool. In this beginner tutorial, you’ll learn how to use Cargo to create, build, and run Rust projects easily.

---

## Step 1: What is Cargo?

Cargo is the tool most Rustaceans use every day. It:
- Creates new projects for you
- Builds and runs your code
- Manages libraries (called "crates")
- Keeps everything tidy

*Tip:* Cargo is like a chef’s recipe manager—it keeps your ingredients (code and libraries) organized and helps you cook (build) your project!

---

## Step 2: Check if You Have Cargo

Open your terminal and type:
```sh
cargo --version
```
If you see a version number, you’re good to go!

---

## Step 3: Create a New Rust Project with Cargo

Let’s use Cargo to start a new project:
```sh
cargo new hello_cargo
cd hello_cargo
```
This makes a folder called `hello_cargo` with everything set up for you.

Inside, you’ll see:
- `Cargo.toml` — Like a recipe card for your project
- `src/main.rs` — Where your code lives

---

## Step 4: Explore the Files

Open `src/main.rs` and you’ll see:
```rust
fn main() {
    println!("Hello, world!");
}
```
This is the same as your first program! Cargo made it for you.

Open `Cargo.toml` and you’ll see:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```
- `[package]` — Info about your project
- `[dependencies]` — List of extra tools (crates) you might use

---

## Step 5: Build and Run with Cargo

Let Cargo do the work:
- To build:
  ```sh
  cargo build
  ```
- To run (builds if needed, then runs):
  ```sh
  cargo run
  ```
- To quickly check your code (no program created):
  ```sh
  cargo check
  ```

*Tip:* `cargo run` is like pressing “play” on your project!

---

## Step 6: Try It Yourself!

- Change the message in `main.rs` and run `cargo run` again.
- Add another `println!` line.
- Try running `cargo build --release` for a super-fast version.

---

## Why Use Cargo?

- Keeps your projects organized
- Makes building and running easy
- Handles libraries for you
- Works the same on Windows, Mac, and Linux

---

## Next Steps for Rust Beginners

- You’re now ready to manage Rust projects like a pro!
- Move on to the next tutorial: [Rust Variables & Mutability](../05-variables-mutability/README.md)
- Explore adding dependencies (crates) in future lessons
- Read the [Official Rust Book: Cargo and Crates.io](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

---

**Cargo is your Rust sidekick. Happy coding! 🦀**
