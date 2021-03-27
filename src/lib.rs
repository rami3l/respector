//! An extension to add [`inspect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect) method to [`Option`] and [`Result`] types.

pub use self::prelude::*;

pub mod prelude {
    pub trait OptionInspector<T> {
        /// Does something with the contained [`Some`] value element of an [`Option`], passing the value on.
        ///
        /// # Examples
        /// ```rust
        /// use respector::prelude::*;
        /// assert_eq!(Some(10).inspect(|x| println!("Some({})", x)), Some(10)); // Prints `Some(10)`.
        /// assert_eq!(None::<i32>.inspect(|x| println!("Some({})", x)), None); // Prints nothing.
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
        /// assert_eq!(
        ///     Ok::<_, ()>(10).inspect(|x| println!("Ok({})", x)),
        ///     Ok(10)
        /// ); // Prints `Ok(10)`.
        /// assert_eq!(
        ///     Err::<(), _>(10).inspect(|x| println!("Ok({:?})", x)),
        ///     Err(10)
        /// ); // Prints nothing.
        /// ```
        fn inspect<F: FnMut(&T)>(self, f: F) -> Result<T, E>;

        /// Does something with the contained [`Err`] value element of a [`Result`], passing the value on.
        ///
        /// # Examples
        /// ```rust
        /// use respector::prelude::*;
        /// assert_eq!(
        ///     Err::<(), _>(10).inspect_err(|x| println!("Err({})", x)),
        ///     Err(10)
        /// ); // Prints `Err(10)`.
        /// assert_eq!(
        ///     Ok::<_, ()>(10).inspect_err(|x| println!("Err({:?})", x)),
        ///     Ok(10)
        /// ); // Prints nothing.
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
