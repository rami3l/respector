//! An extension to add [`inspect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) method to [`Option`] and [`Result`] types.
//! This allows doing something with the value contained in a `Some`, an `Ok` (with `inspect`) or an `Err` (with `inspect_err`) while passing the value on,
//! which can be useful for introducing side effects in debugging, logging, etc.
//!
//!
//! ## Examples
//! ```rust
//! use respector::prelude::*;
//!
//! let some = Some(10);
//! assert_eq!(some.inspect(|x| println!("Some({})", x)), some); // Prints `Some(10)`.
//!
//! let ok = Ok::<_, ()>(10);
//! assert_eq!(ok.inspect(|x| println!("Ok({})", x)), ok); // Prints `Ok(10)`.
//!
//! let err = Err::<(), _>(10);
//! assert_eq!(Err(10).inspect_err(|x| println!("Err({})", x)), err); // Prints `Err(10)`.
//! ```

#![no_std]

pub use self::prelude::*;

pub mod prelude {
    pub trait OptionInspector<T> {
        /// Does something with the contained [`Some`] value element of an [`Option`], passing the value on.
        ///
        /// # Examples
        /// ```rust
        /// use respector::prelude::*;
        ///
        /// let (some, none) = (Some(10), None::<i32>);
        /// assert_eq!(some.inspect(|x| println!("Some({})", x)), some); // Prints `Some(10)`.
        /// assert_eq!(none.inspect(|x| println!("Some({})", x)), none); // Prints nothing.
        /// ```
        fn inspect<F: FnMut(&T)>(self, f: F) -> Option<T>;
    }

    impl<T> OptionInspector<T> for Option<T> {
        fn inspect<F: FnMut(&T)>(self, mut f: F) -> Option<T> {
            self.map(|it| {
                f(&it);
                it
            })
        }
    }

    pub trait ResultInspector<T, E> {
        /// Does something with the contained [`Ok`] value element of a [`Result`], passing the value on.
        ///
        /// # Examples
        /// ```rust
        /// use respector::prelude::*;
        ///
        /// let (ok, err) = (Ok::<_, ()>(10), Err::<(), _>(10));
        /// assert_eq!(ok.inspect(|x| println!("Ok({})", x)), ok); // Prints `Ok(10)`.
        /// assert_eq!(err.inspect(|x| println!("Ok({:?})", x)), err); // Prints nothing.
        /// ```
        fn inspect<F: FnMut(&T)>(self, f: F) -> Result<T, E>;

        /// Does something with the contained [`Err`] value element of a [`Result`], passing the value on.
        ///
        /// # Examples
        /// ```rust
        /// use respector::prelude::*;
        ///
        /// let (ok, err) = (Ok::<_, ()>(10), Err::<(), _>(10));
        /// assert_eq!(err.inspect_err(|x| println!("Err({})", x)), err); // Prints `Err(10)`.
        /// assert_eq!(ok.inspect_err(|x| println!("Err({:?})", x)), ok); // Prints nothing.
        /// ```
        fn inspect_err<F: FnMut(&E)>(self, f: F) -> Result<T, E>;
    }

    impl<T, E> ResultInspector<T, E> for Result<T, E> {
        fn inspect<F: FnMut(&T)>(self, mut f: F) -> Result<T, E> {
            self.map(|it| {
                f(&it);
                it
            })
        }

        fn inspect_err<F: FnMut(&E)>(self, mut f: F) -> Result<T, E> {
            self.map_err(|err| {
                f(&err);
                err
            })
        }
    }
}
