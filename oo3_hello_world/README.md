# Hello, World! in Rust

This section demonstrates how to write and run your first Rust program: the classic "Hello, world!" example, following the [official Rust Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html).

## Creating the Project Directory

It's common to organize your Rust code in a dedicated directory. For example:

- On Linux, macOS, or PowerShell:
  ```sh
  mkdir ~/projects
  cd ~/projects
  mkdir hello_world
  cd hello_world
  ```
- On Windows CMD:
  ```cmd
  mkdir "%USERPROFILE%\projects"
  cd /d "%USERPROFILE%\projects"
  mkdir hello_world
  cd hello_world
  ```

## Writing the Program

Create a file named `main.rs` and add the following code:

```rust
fn main() {
    println!("Hello, world!");
}
```

This program defines a special function called `main`, which is the entry point of every Rust executable. The `println!` macro prints the text `Hello, world!` to the screen.

### Key Points
- `fn main() { ... }` defines the main function.
- `println!` is a macro (note the `!`) that prints text to the terminal.
- Each statement ends with a semicolon (`;`).

## Compiling and Running

To compile and run your program:

- On Linux, macOS, or PowerShell:
  ```sh
  rustc main.rs
  ./main
  ```
- On Windows:
  ```powershell
  rustc main.rs
  .\main
  ```

You should see:
```
Hello, world!
```

## What Happens When You Compile?
- `rustc main.rs` compiles your Rust source code into a binary executable.
- The output is `main` (or `main.exe` on Windows).
- You can run this executable directly, even on a system without Rust installed.

## Anatomy of the Program
- The `main` function is always the entry point.
- Curly braces `{}` define the function body.
- `println!` is a macro for printing output.
- Strings are enclosed in double quotes.

## Congratulations!
If you see `Hello, world!` printed, you've written and run your first Rust program. Welcome to the Rust community!

For more details, see the [official guide](https://doc.rust-lang.org/book/ch01-02-hello-world.html).
