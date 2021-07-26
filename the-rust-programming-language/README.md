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

### 4.2. References and Borrowing

Mutable reference:

```rust
fn main() {
  let mut s = String::from("Nyan");

  mutate(&mut s);

  println!("{}", s);
}

fn mutate(s: &mut String) {
  s.push_str("pasu!");
}
```

> ... you can have only one mutable reference to a particular piece of data in a particular scope.

> A similar rule exists for combining mutable and immutable references.

Not okay:

```rust
let mut s = String::from("Nyanpasu!");

let ref1 = &s;
let ref2 = &s;
let ref3 = &mut s;

println!("{}, {}, and {}", ref1, ref2, ref3);
```

Okay:

```rust
// Not okay.
let mut s = String::from("Nyanpasu!");

let ref1 = &s;
let ref2 = &s;

println!("{}, and {}", ref1, ref2);

let ref3 = &mut s;

println!("{}", ref3);
```

> ... the compiler guarantees that references will never be dangling references.

### 4.3. The Slice Type

```rust
let bytes = s.as_bytes(); // Convert s to an array of bytes.
```

```rust
for (i, &item) in bytes.iter().enumerate() { // Similar to Python.
```

```rust
b' ' // Byte literal of space.
```

String slices:

```rust
let s = String::from("Nyanpasu");

let nyan = &s[0..4];
let pasu = &s[4..8];
let nyanpasu = &s[..8];
let nyanpasu = &s[0..];
let nyanpasu = &s[..];
```

## 5. Using Structs to Structure Related Data

### 5.1. Defining and Instantiating Structs

```rust
struct User {
    username: String,
    email: String,
    // ...
}

// ...

let user = User {
  username: "nyanpasu",
  email: "nyanpasu@example.com",
  // ...
}

let username: "nyanpasu";
let email: "nyanpasu@example.com";
let user = User {
  email,
  username,
}
```

Update syntax:

```rust
let another_user {
  username: "another",
  email: "another@example.com",
  ..user
}
```

Tuple struct:

```rust
struct Colour(u8, u8, u8);

let white_color = Color(255, 255, 255);
```

### 5.2. An Example Program Using Structs

Print structs:

```rust

fn main() {
  let rect = Rectangle {
    height: 24,
    width: 42,
  }

  println!("{:#?}", rect); // #: with pretty print.
}

#[derive(Debug)]
struct Rectangle {
  height: u32,
  width: u32,
}
```

### 5.3. Method Syntax

Defining a method with an `impl` (implementation) block:

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }
}
```

Associated function:

```rust

```

## 6. Enums and Pattern Matching

### 6.1. Defining an Enum

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

// With types.
enum Direction {
  Up(String),
  Down(String),
  Left(String),
  Right(String),
}

// Anonymous struct
enum Action {
  Move { x: i32, y: i32 },
}
```

Enums can also have methods, defined using `impl`.

Standard `Option` `enum`:

```rust
enum Option<T> {
  Some(T),
  None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

### 6.2. The `match` Control Flow Operator

> When the match expression executes, it compares the resulting value against the pattern of each arm, in order. 

> Matches in Rust are exhaustive.

Match all with the `_` placeholder:

```rust
let some_u8_value = 0u8;

match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  _ => println!("Not one or three."),
}
```

### 6.3. Concise Control Flow with `if let`

```rust
if let Some(3) = some_u8_value {
  println!("three");
} else { // Optional else.
  
}
```

`if let` is syntax sugar for `match`ing one pattern while ignoring all others.

## 7. Managing Growing Projects with Packages, Crates, and Modules

### 7.1. Packages and Crates

> A package must contain zero or one library crates, and no more

> ... Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.

### 7.2. Defining Modules to Control Scope and Privacy

Creating a library:

```sh
cargo new --lib restaurant
```

Module definition:

```rust
mod some_module {
  // ...
}
```

> Modules can also hold definitions for other items, such as structs, enums, constants, traits [and functions]

### 7.3. Paths for Referring to an Item in the Module Tree

> Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary.

Making descendent modules public:

```rust
mod some_module {
  // ...
  mod private_child_module {
    // ...
  }

  pub mod public_child_module {
    // ..
  }
}
```

> [The `super` keyword] is like starting a filesystem path with the `..` syntax.

```rust
mod some_module {
  // ...
  mod private_child_module {
    // ..

    super::outer_function();
  }
}

fn outer_function() {
  // ...
}
```

Similarly, we can also make structs and enums public with the `pub` keyword. Note that individual fields are not public by default and require the `pub` keyword, too, where necessary.

A struct with one or more private field requires a public constructor, otherwise one would not be able to instantiate the struct.

All variants of a public enum is automatically made public, too.

### 7.4. Bringing Paths Into Scope with the use Keyword

```rust
// Using the previous example.
use crate::some_module::public_child_module;

pub fn some_function() {
  public_child_module::some_method();
}
```

> Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.

We can also use the `self` keyword:

```rust
// Using the previous example.
use self::some_module::public_child_module;

pub fn some_function() {
  public_child_module::some_method();
}
```

Alias using the `as` keyword:

```rust
use std::io::Result as IoResult;
```

Re-exporting:

```rust
pub use crate::some_module::public_child_module; // External code now has access to public_child_module.
```

Nested paths and importing all:

```rust
use std::{cmp::Ordering, io};

use std::io::{self, Write};

use std::collections::*; // glob operator
```

### 7.5. Separating Modules into Different Files

Importing module form another file:

```rust
// src/hello.rs
pub mod world {
  pub fn print() {
    println!("Hello, world!");
  }
}

// src/main.rs
mod hello;

fn main() {
  hello::world::print();
}
```

## 8. Common Collections

### 8.1. Storing Lists of Values with Vectors

> Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

```rust
let v: Vec<i32> = Vec::new();

let v = vec![1, 2, 3]; // Type infrencin

v.push(4); // Adding element.

println!("The value at index {} of v is: {}", 3, v[3]); // 4. Reading element.

let value = v.get(3);

match value {
    Some(x) => println!("The value at index 3 of v is: {}", x),
    None => println!("No value found at the given index!")
};
```

Iterating over values:

```rust
let v = vec![1, 2, 3];

for i in &v {
  println!("{}", i);
}
```

Iterate with mutation:

```rust
let mut v = vec![1, 2, 3];

for i in &mut v {
  *i += 42;
}
```

Enums can be used to create vectors containing complex values while fulfilling the same-type requirement.

### 8.2. Storing UTF-8 Encoded Text with Strings

> The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

```rust
let s1 = String::from("nyanpasu");
let s2 = "nyanpasu".to_string();
let mut s3 = String::from("nyan");
let mut s4 = String::from("nyan");
let s5 = format!("{}{}", "nyan", "pasu");

s3.push_str("pasu"); // Note that "pasu" is of type str. This is coercion.
s4 += "pasu"; // Note that "pasu" is of type str. This is coercion.

println!("s1 is equal to s2: {}", s1 == s2); // true
println!("s1 is equal to s3: {}", s1 == s3); // true
println!("s1 is equal to s4: {}", s1 == s4); // true
println!("s1 is equal to s4: {}", s1 == s5); // true
```

> Rust strings don’t support indexing.

Consider the following examples:

```rust
let hello = String::from("Hola"); // 4 bytes.
let hello = String::from("Здравствуйте"); // 24 bytes.
let hello = String::from("नमस्ते"); // 18 bytes.
```