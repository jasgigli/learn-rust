# 🚀 Installing Rust: Your First Tool!

## Overview

Before you can build anything in Rust, you need to set up your toolbox. Installing Rust is quick and easy—no experience required! Think of this as getting your chef’s knife before you start cooking.

---

## Step 1: Install Rust (with rustup)

Rust uses a tool called **rustup** to manage everything for you. It’s like an app store for Rust tools!

### Windows
- Go to [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and click the big button to download the installer.
- Run the installer and follow the prompts. (If it asks about Visual Studio, say yes—it helps Rust build programs on Windows.)

### Mac or Linux
- Open your Terminal app.
- Copy and paste this command, then press Enter:

  ```sh
  curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  ```
- Follow the on-screen instructions.

*Analogy:* Installing Rust is like setting up your kitchen before you start cooking. You only need to do it once!

---

## Step 2: Check That Rust Works

Open your terminal (or Command Prompt/PowerShell on Windows) and type:

```sh
rustc --version
```

If you see something like `rustc 1.XX.X (date)`, you’re ready!

---

## Step 3: Your Rust Toolbox

- **rustc**: The Rust compiler (turns your code into programs)
- **cargo**: The project manager (helps you build, run, and share Rust projects)
- **rustup**: The tool installer/updater

You get all of these automatically!

---

## Step 4: Next Steps

- Try running `rustup doc` to open Rust’s local documentation in your browser.
- Choose a code editor you like (VS Code, Sublime, etc.). Most editors have Rust plugins for helpful features.
- If you run into trouble, check your system’s PATH or visit the [Rust community](https://www.rust-lang.org/community) for help.

---

## Troubleshooting (If Needed)

- If `rustc --version` doesn’t work, restart your terminal or computer.
- Make sure Rust is in your PATH (search online for “add to PATH” if unsure).
- Still stuck? The [official guide](https://doc.rust-lang.org/book/ch01-01-installation.html) has more tips.

---

## What’s Next?

You’ve got Rust installed! 🎉

- Head to the next tutorial: [Hello World](../oo3_hello_world/README.md)
- Try the [Rust Playground](https://play.rust-lang.org/) to write code in your browser
- Explore the [Rust Book](https://doc.rust-lang.org/book/)

---

**You’re ready to write your first Rust program. Let’s do it! 🦀**
