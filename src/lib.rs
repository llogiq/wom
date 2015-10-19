//! Write-only Memory for Rust
//!
//! This is a wrapper type to disallow reading from the wrapped type

use std::convert::{AsRef, AsMut};

/// Make `T` Write-Only
pub struct Wom<T> {
    #[doc(hidden)]
    _inner: T
}

impl<T> Wom<T> {
    /// Transmute this value into the wrapped value
    ///
    /// # Examples
    ///
    /// ```
    ///# use wom::wom;
    /// assert_eq!(1, wom(1u32).into_inner());
    /// ```
    pub fn into_inner(self) -> T {
        self._inner
    }
}

/// Create a Wom handle on our `T`.
pub fn wom<T>(t: T) -> Wom<T> {
    Wom { _inner: t }
}

impl<T: Sized> AsRef<T> for Wom<T> {
    /// Getting an immutable value out should panic.
    /// # Examples
    ///
    /// ```should_panic
    ///# use wom::wom;
    /// wom(1u32).as_ref();
    /// ```
    fn as_ref(&self) -> &T {
        panic!("readably referencing write-only memory");
    }
}

impl<T: Sized> AsMut<T> for Wom<T> {
    /// Transmute this value into the wrapped value
    ///
    /// # Examples
    ///
    /// ```
    ///# use wom::wom;
    /// let x = [1u32];
    /// let mut w = wom(x);
    /// w.as_mut()[0] = 2;
    /// assert_eq!(2, w.into_inner()[0]);
    /// ```
    fn as_mut(&mut self) -> &mut T {
        &mut self._inner
    }
}

//TODO: Index, IndexMut
