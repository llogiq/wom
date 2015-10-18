//! Write-only Memory for Rust
//!
//! This is a wrapper type to disallow reading from the wrapped type

use std::convert::{AsRef, AsMut}

/// Make `T` Write-Only
pub struct<T> Wom<T> {
    #[doc(hidden)]
    _inner: T
}

impl<T: ?Sized> AsRef<T> for Wom<T> {
    fn as_ref(&self) -> &T {
        panic!("readably referencing write-only memory");
    }
}

impl<T: ?Sized> AsMut<T> for Wom<T> {
    fn as_mut(&mut self) -> &mut T {
        self._inner
    }
}

//TODO: Index, IndexMut
