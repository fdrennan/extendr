use libR_sys::{R_IsNA, R_NaReal};
use crate::Bool;
/// Return true if this primitive is NA.
pub trait CanBeNA {
    fn is_na(&self) -> bool;
    fn na() -> Self;
}

impl CanBeNA for f64 {
    fn is_na(&self) -> bool {
        unsafe { R_IsNA(*self) != 0 }
    }

    fn na() -> f64 {
        unsafe{R_NaReal}
    }
}

impl CanBeNA for i32 {
    fn is_na(&self) -> bool {
        *self == i32::na()
    }

    fn na() -> i32 {
        i32::MIN
    }
}

impl CanBeNA for Bool {
    fn is_na(&self) -> bool {
        self.0 == Bool::na().0
    }

    fn na() -> Bool {
        Bool::from(i32::MIN)
    }
}

impl CanBeNA for &str {
    /// Check for NA in a string by address.
    fn is_na(&self) -> bool {
        self.as_ptr() == <&str>::na().as_ptr()
    }

    /// Special "NA" string that represents null strings.
    /// ```
    /// use extendr_api::prelude::*;
    /// test! {
    ///     assert_ne!(<&str>::na().as_ptr(), "NA".as_ptr());
    ///     assert_eq!(<&str>::na(), "NA");
    ///     assert_eq!("NA".is_na(), false);
    ///     assert_eq!(<&str>::na().is_na(), true);
    /// }
    /// ```
    fn na() -> Self {
        unsafe { std::str::from_utf8_unchecked(&[b'N', b'A']) }
    }
}
