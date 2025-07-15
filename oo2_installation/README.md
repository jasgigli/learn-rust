# Installing Rust

This guide will help you install Rust on your system, following the official instructions from the [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html).

## 1. Installing Rust with rustup

Rust is installed and managed using `rustup`, a command-line tool for managing Rust versions and associated tools.

### Windows
- Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions.
- During installation, you may be prompted to install Visual Studio (for the required linker and native libraries). For more help, see [Windows MSVC Installation Guide](https://rust-lang.github.io/rustup/installation/windows-msvc.html).

### Linux and macOS
Open a terminal and run:

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- You may be prompted for your password.
- If you encounter linker errors, install a C compiler (e.g., `build-essential` on Ubuntu or `xcode-select --install` on macOS).

## 2. Verifying Installation
After installation, check that Rust is installed by running:

```sh
rustc --version
```

You should see output like:

```
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

## 3. Updating and Uninstalling Rust
- To update Rust:
  ```sh
  rustup update
  ```
- To uninstall Rust:
  ```sh
  rustup self uninstall
  ```

## 4. Local Documentation
Rust installs local documentation. Open it with:

```sh
rustup doc
```

## 5. Editors and IDEs
You can use any text editor or IDE. Many have Rust support. See the [Rust tools page](https://www.rust-lang.org/tools) for recommendations.

## 6. Troubleshooting
- Ensure Rust is in your system `PATH`.
- On Windows CMD:
  ```cmd
  echo %PATH%
  ```
- On PowerShell:
  ```powershell
  echo $env:Path
  ```
- On Linux/macOS:
  ```sh
  echo $PATH
  ```

If you need help, visit the [Rust community page](https://www.rust-lang.org/community).

## 7. Working Offline
To cache dependencies for offline use:

```sh
cargo new get-dependencies
cd get-dependencies
cargo add rand@0.8.5 trpl@0.2.0
```

You can then use the `--offline` flag with Cargo commands.

---

For more details, see the [official installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html).
