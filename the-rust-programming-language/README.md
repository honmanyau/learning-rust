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