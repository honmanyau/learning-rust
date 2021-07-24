# The Rust Programming Language

This file contains crude notes for points that I find important or interesting in the book [The Rust Programming Language](https://doc.rust-lang.org/book).

## Introduction

> High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict.

> Rust gives you the option to control low-level details...

Ferris is cute, but I'm not sure that the pose for "This code does not produce the desired behavior." makes sense to me. :(

## 1. Getting Started

### 1.1 Installation

```sh
rustup update

rustup self uninstall

rustc --version

rustup doc
```

### 1.2 Hello, world!

> The main function is special: it is always the first code that runs in every executable Rust program.

Use `rustfmt` for formatting.

> Rust is an ahead-of-time compiled language.

This is gold:

> Everything is a trade-off in language design.

### 1.3 Hello, Cargo!

> Rustaceans

xD...

```sh
cargo --version

cargo new
cargo build [--release]
cargo run
cargo check # Check if code can compile. Does not produce executable.
```

## 2. Programming a Guessing Game

[Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html): A small set of predefined imports.

> In Rust, variables are immutable by default.

```rust
let snowbell = String::new(); // immutable
let mut cookie = String::new(); // mutable
```

Using `::` with a type indicates an *associated function* (static method).

Note that instead of:

```rust
use std::io;

io::stdin()
// ...
```

We can also do:

```rust
std::io::stdin()
// ...
```

The `&` symbol indicates an argument is a *reference*, like `*` in C?

Reference in Rust are immutable by default, therefore in the exercise we write `io::stdin().read_line(&mut guess)`.

String formatting:

```rust
println!("Your number is: {}", value);
```

> Cargo understands Semantic Versioning 

Rust crate registry: https://crates.io

```sh
cargo update
```

```rust
use rand::Rng // Rng is a trait. Not covered here yet, more in Chapter 14.
```

Range expressions:

```rust
1..101 // Between 1 (inclusive) and 101 (exclusive).
1..=100 // Both ends inclusive.
```

```sh
cargo doc --open # Build dependencies for all local dependencies and opens in browser. This is way cool.
```

A `match` expression comprises arms, each arm contains a pattern for matching.

Rust allows shadowing of variables:

```rust
let mut guess = String::new();

// ...

let guess: u32 = guess.parse().expect("Please enter a number!");
```

Handling the `Result` type:

```rust
let guess: u32 = guess.trim().parse() {
  Ok(num) => num,
  Err(_) => continue, // _ is a catch all value.
}
```

## 3. Common Programming Concepts

### 3.1 Variables and Mutability

> Constants aren’t just immutable by default—they’re always immutable.

> ... constants may be set only to a constant expression...

Like in JavaScript:

```rust
const // ... Must have data type annotation.
let // ...
```

### 3.2 Data Types

> Rust is a statically typed language

Rust has signed and unsigned integer types from 8-bit bit up to 128-bit:

```rust
let x: i8 = -128;
let x: u8 = 127;
let x: isize = 42; // Depends on CPU architecture.
```

Floating point:

```rust
let x: f32 = 3.2;
let x: f64 = 6.4;
```

> Rust’s char type is four bytes in size and represents a Unicode Scalar Value.

> Rust has two primitive compound types: tuples and arrays.

```rust
let tup: (i32, f64, u8) = (42, 4.2, 1);
```

Tuple destructuring:

```rust
let (x, y, z) = tup; // Using tup from the previous example.

println!(x); // 42
println!(tup.0); // 42
println!(tup.1); // 4.2
println!(tup.2); // 1
```

Rust arrays are fixed in length.

```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // [3, 3, 3, 3, 3]

print(a[0]); // 3
print(a[1]); // 3
```

We can use typed parameter in Rust:

```rust
fn fib(x: i32) {
  // ...
}
```

3.3. Functions

> Statements do not return values.

3.5. Flow Control

It looks like `rustfmt` doesn't like parentheses around `if` conditions.

Consider using `match` when there are too many `if`-`else` statements involved.

> ... blocks of code evaluate to the last expression in them.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

println!("The value of number is: {}", number); // 5
```

Loops: `loop`, `while` and `for`.

We can return a value inside a loop as follows:

```rust
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
}; // 20
```

For loop in Rust:

```rust
for number in (1..4) {
    println!("{}", number);
}
// 1
// 2
// 3
```

## 4. Understanding Ownership

### 4.1. What is Ownership?

> Each value in Rust has a variable that’s called its owner.
> There can only be one owner at a time.
> When the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    let s1 = String::from("Nyanpasu!");
    let s2 = s1; // s1 is no longer valid. This is known as `move` in Rust.

    println!("{}", s1);
}
```

> Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

To create a deep copy we `clone`:

```rust
fn main() {
    let s1 = String::from("Nyanpasu!");
    let s2 = s1.clone();

    println!("{} {}", s1, s2); // Nyanpasu! Nyanpasu!
}
```

> Passing a variable to a function will move or copy, just as assignment does.

> Returning values can also transfer ownership

