# Module 06: Monadic Worlds

## Introduction

This module is the first that'll actually look at the Rust Standard Library's constructs. The
standard library is a separate crate automatically included in every Rust project. There is ways to
disable it, but this is beyond the scope of this piscine.

The Rust standard library, while not as large as C++'s, exports lots of useful types and constructs
to help you create efficient software. This module will introduce you to some of them.

Always remember: a monad is just a monoid in the cateogry of endofunctors.

## General Rules

Any exercise you turn in must compile using the `cargo` package manager, either with `cargo run`
if the subject requires a *program*, or with `cargo test` otherwise. Only dependencies specified
in the `allowed dependencies` section are allowed. Only symbols specified in the `allowed symbols`
section are allowed. Every exercise must be part of a virtual Cargo workspace, a single
`workspace.members` table must be declared for the whole module.

Everything must compile *without warnings* with the `rustc` compiler available on the school's
machines without additional options. You are allowed to use attributes to modify lint levels, but
you must be able to explain why you did so. You are *not* allowed to use `unsafe` code anywere in
your code.

## Exercise 00: Maybe

```txt
turn-in directory:
    ex00/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::{assert*, matches, panic}
```

The Rust type system can be used to represent optional values. Create an `enum` that can either
contain `Something(T)`, or `Nothing`. That type must be named `Maybe<T>`.

You type must implement a method called `get_or_panic`. That function must either return the
value stored in the input `Maybe<T>` instance, or panic if it contains nothing.

You must also provide two methods to quickly test whether a `Maybe<T>` instance contains
something or not.

```rust
impl<T> Maybe<T> {
    fn get_or_panic(self) -> T;

    fn contains_something(&self) -> bool;
    fn contains_nothing(&self) -> bool;
}
```

You must add tests to prove your function works as expected. You *have to* add a test that shows
that the `get_or_panic` function panics when the instance it is given contains no value.

## Exercise 01: Integer Square Root

```txt
turn-in directory:
    ex01/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    std::option::Option  std::{assert*}  u32::checked_mul
```

Because it is such a common pattern, the Rust Standard Library already defines the `Maybe<T>` type
for you. It is named `Option<T>`. That type is used to encode the potential non-existence of a
value.

Create a **function** that computes the square root of an integer. If the input of the function is
a perfect square, then its square root is returned. Otherwise, `None` is returned.

```rust
fn int_sqrt(n: u32) -> Option<u32>;
```

Example:

```rust
assert_eq!(int_sqrt(16), Some(4));
assert_eq!(int_sqrt(15), None);
```

You must provide tests to prove the function you wrote is indeed correct.

## Exercise 02: Niche Optimization

```txt
turn-in directory:
    ex02/

files to turn in:
    src/main.rs  Cargo.toml
```

Copy the following `main` function:

```rust
fn main() {
    dbg!(std::mem::size_of::<usize>());
    dbg!(std::mem::size_of::<Option<usize>>());
    dbg!(std::mem::size_of::<&u8>());
    dbg!(std::mem::size_of::<Option<&u8>>());
}
```

Can you explain why `Option<usize>` takes more space than `usize` whereas `Option<&u8>` takes as
much memory as a regular `&u8`? You will be asked during defense.

## Exercise 03: Handling Errors

```txt
turn-in directory:
    ex03/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::println  std::eprintln  str::parse  std::result::Result
    std::num::{ParseIntError, IntErrorKind}  std::env::args
    std::process::ExitCode
```

Create a **program** that takes exactly one argument. If no arguments (or more than one) are
passed, the program prints an error message but *does not panic*.

The single argument is parsed into an `i32` instance. If the convertion is a success, the function
prints a message indicating that the convertion is a success. If an error occurs, a message
describing the error is written.

Example:

```
>_ cargo run -- a b c
too many arguments: exactly one argument is expected
>_ cargo run -- '1234'
success
>_ cargo run -- 'a'
invalid digit: a non numerical character has been found
>_ cargo run -- '1111111111111111111111111111'
positive overflow: the provided value overflows the type `i32`
```

You must not create your own "atoi" function, use what Rust gives you!

## Exercise 04: `Option<T>` To `Result<T>`

```txt
turn-in directory:
    ex04/

files to turn in:
    src/lib.rs  Cargo.toml

allowed symbols:
    u32::{checked_add, checked_mul}  std::result::Result  std::{assert*}
```

Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.
Something has gone wrong, return the error. Otherwise continue. Something has gone wrong, return
the error. Otherwise continue. Something has gone wrong, return the error. Otherwise continue.

```rust
#[derive(Debug, PartialEq)]
enum ComputeError {
    CantMultiply,
    CantAdd,
}
```

Write a **function** that:

* Takes a number as an input.
* Multiplies that number by `42`.
* Adds `100`.
* Returns the result.

The function is *never* allowed to panic, nor is it allowed to return an incorrect value. You must
write write this whole function without using a single semicolon `";"`.

The function must be prototyped like so:

```rust
fn compute(input: u32) -> Result<u32, ComputeError>;
```

Example:

```rust
assert_eq!(parse_and_compute(12), Ok(12 * 42 + 100));
assert_eq!(parse_and_compute(1000000000), Err(ComputeError::CantMultiply));
```

*You have you write tests. ~*

## Exercise 05: What Time Is It?

```txt
turn-in directory:
    ex05/

files to turn in:
    src/main.rs  Cargo.toml

allowed symbols:
    std::str::FromStr  std::fmt::{Display, Debug, Formatter}
    str::*  std::result::Result  std::{write, println}
```

Create a type named `Time` responsible for storing, well, a time.

```rust
fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
```

Implement the right traits such that the above code produces the following output.

```txt
>_ cargo run
12 hours, 20 minutes
15 hours, 14 minutes
error: missing ':'
error: invalid length
error: invalid number
```
