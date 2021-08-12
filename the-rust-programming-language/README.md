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

> ... Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.

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

### 8.3. Storing Keys with Associated Values in Hash Maps

> Just like vectors, hash maps store their data on the heap.

> Like vectors, hash maps are homogeneous.

```rust
let mut nonsensical = HashMap::new();

nonsensical.insert(String::from("nyan"), 24);
nonsensical.insert(String::from("pasu"), 42);

let words = vec!("nyan", "pasu");
let nums = vec!(24, 42);
let nonsensical: HashMap<_, _> =
    words.into_iter().zip(nums.into_iter()).collect();

println!("{:?}", nonsensical);
```

> For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values.

Accessing values in a hash map:

```rust
let value = nonsensical.get("nyan");

match value {
  Some(x) => println!("{}", x),
  None => println!("No value found")
}

for (key, value) in &nonsensical {
  println!("{}: {}", key, value);
}
```

Overwriting values:

```rust
nonsensical.insert("nyan", 42);
nonsensical.insert("pasu", 24);
```

Insert if key has no value:

```rust
nonsensical.entry("nyanpasu").or_insert(4224);
```

> By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.

## 9. Error Handling

### 9.1. Unrecoverable Errors with panic!

> When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

To abort on panic without unwinding:

```toml
[profile.release]
panic = 'abort'
```

Stack trace for `panic!`:

```sh
RUST_BACKTRACE=1 cargo run
```

### 9.2. Recoverable Errors with Result

Error handling example:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let filename = "nyanpasu.txt";
  let f = File:: open(filename);
  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(filename) {
        Ok(created) => created,
        Err(e) => panic!("Problem creating {}: {:?}", filename, e)
      },
      other_error => {
        panic!("Problem opening {}: {:?}", filename, other_error)
      }
    }
  };
}
```

An example of returning value of `Ok` or panic automatically:

```rust
let f = File::open("nyanpasu.txt").unwrap();
```

Or use `expect` to also provide a custom message:

```rust
let f = File::open("nyanpasu.txt").expect("Failed to open nyanpasu.txt");
```

Returning `Result` in a function:

```rust
fn read_some_file() -> Result<String, io::Error> {
  // Code for reading a file ...

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
```

The `?` operator:

```rust
fn read_some_file() -> Result<String, io::Error> {
  let mut f = File::open("nyanpasu.txt")?;
  let mut s = String::new();

  f.read_to_string(&mut s)?;
  Ok(s)
}
```

Chaining after `?`:

```rust
fn read_some_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("nyanpasu.txt")?f.read_to_string(&mut s)?;
  Ok(s)
}
```

One-liner that works the same as the functions above:

```rust
fn read_some_file() -> Result<String, io::Error> {
  fs::read_to_string("nyanpasu.txt")
}
```

### 9.3. To panic! or Not To panic!

> It’s advisable to have your code panic when it’s possible that your code could end up in a bad state [and]:
>
> * The bad state is not something that’s expected to happen occasionally.
> * Your code after this point needs to rely on not being in this bad state.
> * There’s not a good way to encode this information in the types you use.

> However, when failure is expected, it’s more appropriate to return a Result than to make a panic!


## 10. Generic Types, Traits, and Lifetimes

### 10.1. Generic Data Types

```rust
fn largest<T>(list: &[T]) -> T {
  // ...
}
```

Using generic with structs and methods:

```rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point { x: 4, y: 2 };
  let float = Point { x: 4.2, y: 2.4 };
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
```

> By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.

Compare the above with a method implemented using a concrete type:

```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```


And in enums:

```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

> Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

> Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.

### 10.2. Traits: Defining Shared Behavior

> Traits are similar to a feature often called interfaces in other languages, although with some differences.

Defining a trait:

```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

Implementing a trait:

```rust
pub struct NewsArticle {
  // ...
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    // ...
  }
}
```

> But we can’t implement external traits on external types... This restriction is part of a property of programs called coherence, and more specifically the orphan rule...


Default behaviour:

```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

impl Summary for NewsArticle {}
```

Trait as parameters:

```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```

The function above accepts any type that implements the `Summary` trait. The syntax used is actually syntax sugar for the following (trait bound syntax):

```rust
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
```

Combining traits in one trait bound:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
  // ...
}
```

Using the `where` clause with trait bounds:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
  // ...
}
```

We can also use the `impl Trait` syntax as a return type, **but we can only return a single type** (we can use trait objects to get around this, see notes for Chapter 17).

Conditional trait implementation by matching trait bounds:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
  // ...
}

impl<T: Display + PartialOrd> Pair<T> {
  fn some_function(&self) {
    // ...
  }
}
```

Blanket implementation, for example, implement the `ToString` type for any type that implements `Display`:

```rust
impl<T: Display> ToString for T {
  // ...
}
```

### 10.3. Validating References with Lifetimes

> ... every reference in Rust has a *lifetime*, which is the scope for which that reference is valid.

> Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

> The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid.

Lifetime annotation syntax:

```rust
&'a i32
&'a mut i32
```

The following annotation tells Rust that both references `x` and `y` have the same lifetime. `'a`, as the return value. The code would not compile without lifetime annotation because the compiler cannot tell in advance (in fact, we can't either) whether `x` or `y` would be returned a therefore what their lifetimes should be w.r.t. the return value.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

This also compiles and highlights that lifetimes are about relationships:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}
```

> Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. **Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.**

Structs can also hold references, and in that case they require lifetime annotation:

```rust
struct SomeStruct<'a> {
    part: &'a str,
}
```

> \[Some\] situations were predictable and followed a few deterministic patterns... The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.

The following code, which doesn't have lifetime annotations, compiles because of lifetime elision rules:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

> Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

The `'static` lifetime "means that a reference can live for the entire duration of the program."

```rust
let s: &'static str = "Wool carpet.";
```

