# Variables and Mutability in Rust

This section explains how variables and mutability work in Rust, following the [official Rust Book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).

## Immutability by Default

In Rust, variables are **immutable by default**. Once a value is bound to a variable name, you cannot change that value unless you explicitly make the variable mutable. This design encourages safety and helps prevent bugs related to unexpected value changes.

Example (immutable variable):
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // This line would cause a compile-time error
}
```
If you try to assign a new value to `x`, the compiler will produce an error:
> cannot assign twice to immutable variable `x`

## Making Variables Mutable

To allow a variable's value to change, use the `mut` keyword:
```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
Now, `x` can be changed from `5` to `6`.

## Constants

Constants are always immutable and must have their type annotated. They are declared with the `const` keyword and can be set only to constant expressions:
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
- Constants can be declared in any scope, including global scope.
- Naming convention: all uppercase with underscores.

## Shadowing

Rust allows you to declare a new variable with the same name as a previous variable. This is called **shadowing**. The new variable overshadows the previous one within its scope.

Example:
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```
Output:
```
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from `mut`:
- With `mut`, you can change the value but not the type.
- With shadowing, you can change both the value and the type:
```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len(); // Now spaces is a number
}
```

If you try to change the type with `mut`, you'll get a compile-time error.

## Summary
- Variables are immutable by default; use `mut` for mutability.
- Constants are always immutable and require type annotations.
- Shadowing lets you reuse variable names and even change types.

For more details, see the [official guide](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html).
