use super::scalar::Rfloat;
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's double vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| (i as f64).into()).collect::<Doubles>();
///     vec.iter_mut().for_each(|v| *v = *v + 10.0);
///     assert_eq!(vec.elt(0), 10.0);
///     let sum = vec.iter().sum::<Rfloat>();
///     assert_eq!(sum, 60.0);
/// }
/// ```  
#[derive(Debug, PartialEq, Clone)]
pub struct Doubles {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    vector_type: Doubles,
    scalar_type: Rfloat,
    primitive_type: f64,
    r_prefix: REAL,
    SEXP: REALSXP,
    doc_name: double,
    altrep_constructor: make_altreal_from_iterator,
);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Doubles = (0..3).map(|i| (i as f64).into()).collect();
            assert_eq!(vec, Doubles::from_values([0.0, 1.0, 2.0]));
        }
    }
    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Doubles::from_values([0.0, 1.0, 2.0, 3.0]);
            vec.iter_mut().for_each(|v| *v = *v + 1.0);
            assert_eq!(vec, Doubles::from_values([1.0, 2.0, 3.0, 4.0]));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Doubles::from_values([0.0, 1.0, 2.0, 3.0]);
            assert_eq!(vec.iter().sum::<Rfloat>(), 6.0);
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Doubles::from_values((0..3).map(|i| 2.0 - i as f64));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([2.0, 1.0, 0.0]));
            assert_eq!(vec.elt(1), 1.0);
            let mut dest = [0.0.into(); 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [1.0, 0.0]);
        }
    }
    #[test]
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors are lazy ALTREP objects.
            let vec = Doubles::from_values((0..1000000000).map(|x| x as f64));
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), 12345678.0);
            let mut dest = [0.0.into(); 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [12345678.0, 12345679.0]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Doubles::new(10);
            assert_eq!(vec.is_real(), true);
            assert_eq!(vec.len(), 10);
        }
    }
}

// TODO: this should be a trait.
impl Doubles {
    pub fn set_elt(&mut self, index: usize, val: Rfloat) {
        unsafe {
            SET_REAL_ELT(self.get(), index as R_xlen_t, val.inner());
        }
    }
}

impl Deref for Doubles {
    type Target = [Rfloat];

    /// Treat Doubles as if it is a slice, like Vec<Rfloat>
    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = DATAPTR_RO(self.get()) as *const Rfloat;
            std::slice::from_raw_parts(ptr, self.len())
        }
    }
}

impl DerefMut for Doubles {
    /// Treat Doubles as if it is a mutable slice, like Vec<Rfloat>
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = DATAPTR(self.get()) as *mut Rfloat;
            std::slice::from_raw_parts_mut(ptr, self.len())
        }
    }
}
