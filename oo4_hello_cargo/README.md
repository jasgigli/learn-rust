[⬅️ Back to Main Table of Contents](../README.md)

# Hello, Cargo!

This section introduces **Cargo**, Rust’s build system and package manager, following the [official Rust Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).

## What is Cargo?

Cargo is the tool most Rust developers use to manage their projects. It handles building your code, downloading and compiling dependencies, and more. Cargo makes it easy to manage even complex Rust projects.

## Checking if Cargo is Installed

Cargo is included with Rust if you used the official installer. To check if Cargo is installed, run:

```sh
cargo --version
```

If you see a version number, Cargo is ready to use!

## Creating a New Project with Cargo

To create a new project, use:

```sh
cargo new hello_cargo
cd hello_cargo
```

This creates a new directory called `hello_cargo` with the following structure:

- `Cargo.toml` — Project configuration in TOML format
- `src/main.rs` — Main source file

Cargo also initializes a Git repository by default.

## Understanding Cargo.toml

The `Cargo.toml` file contains metadata about your project:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `[package]` section: Project info
- `[dependencies]` section: List of external crates (libraries) your project uses

## Writing Code with Cargo

Cargo places your code in `src/main.rs`. By default, it contains:

```rust
fn main() {
    println!("Hello, world!");
}
```

## Building and Running with Cargo

To build your project:

```sh
cargo build
```

- The executable is placed in `target/debug/hello_cargo` (or `hello_cargo.exe` on Windows).

To run your project in one step:

```sh
cargo run
```

- This builds (if needed) and runs the program.

To quickly check if your code compiles (without producing an executable):

```sh
cargo check
```

- This is faster and useful for development.

## Release Builds

For optimized builds, use:

```sh
cargo build --release
```

- The executable will be in `target/release/`.

## Why Use Cargo?

- Simplifies project setup and management
- Handles dependencies automatically
- Organizes code and configuration
- Provides consistent commands across platforms

## Summary

- Use `cargo new` to create projects
- Use `cargo build` to compile
- Use `cargo run` to build and run
- Use `cargo check` to quickly check code
- Use `cargo build --release` for optimized builds

Cargo is the standard way to manage Rust projects. For more details, see the [official guide](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).
