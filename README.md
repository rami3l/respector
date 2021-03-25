# respector

![Downloads](https://img.shields.io/crates/d/respector)
![License](https://img.shields.io/crates/l/respector)
[![crates.io](https://img.shields.io/crates/v/respector?logo=rust)](https://crates.io/crates/respector)
[![docs.rs](https://docs.rs/respector/badge.svg)](https://docs.rs/respector)

An extension to add [`inspect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) method to `Option` and `Result` types.

## Usage

````rust
use respector::prelude::*;

assert_eq!(
    Some(10).inspect(|x| println!("Some({})", x)),
    Some(10)
); // Prints `Some(10)`.

assert_eq!(
    Ok::<_, ()>(10).inspect(|x| println!("Ok({})", x)),
    Ok(10)
); // Prints `Ok(10)`.

assert_eq!(
    Err::<(), _>(10).inspect_err(|x| println!("Err({})", x)),
    Err(10)
); // Prints `Err(10)`.
```
````
