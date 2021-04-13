# respector

[![Downloads](https://img.shields.io/crates/d/respector)](https://crates.io/crates/respector)
[![License](https://img.shields.io/crates/l/respector)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/respector?logo=rust)](https://crates.io/crates/respector)
[![docs.rs](https://docs.rs/respector/badge.svg)](https://docs.rs/respector)

An extension to add [`inspect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) method to `Option` and `Result` types.

This allows doing something with the value contained in a `Some`, an `Ok` (with `inspect`) or an `Err` (with `inspect_err`) while passing the value on,
which can be useful for introducing side effects in debugging, logging, etc.

## Usage

```rust
use respector::prelude::*;

let some = Some(10);
assert_eq!(some.inspect(|x| println!("Some({})", x)), some); // Prints `Some(10)`.

let ok = Ok::<_, ()>(10);
assert_eq!(ok.inspect(|x| println!("Ok({})", x)), ok); // Prints `Ok(10)`.

let err = Err::<(), _>(10);
assert_eq!(Err(10).inspect_err(|x| println!("Err({})", x)), err); // Prints `Err(10)`.
```
