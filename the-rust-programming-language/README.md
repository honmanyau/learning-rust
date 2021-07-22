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