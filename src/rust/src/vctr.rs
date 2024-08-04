use crate::helpers::*;
use crate::VecUsize;
use extendr_api::prelude::*;

// This is the trait that all Rust objects that want to
// be treated as R vectors will need to implement
pub trait Rvctr {
    fn class() -> &'static str;
    fn show(&self) -> Strings;
    fn length(&self) -> Rint;
    fn subset(self, idx: Integers) -> Self;
    fn extend(self, y: Self) -> Self;
}

// The `Vctr` Struct is intended to be a simple wrapper that
// contains any custom types that is defined by the developer.
// The Vctr struct requires that any object contained by it must
// implement the Rvctr trait which is minimal
// //implement Vctr trait
impl Rvctr for VecUsize {
    fn length(&self) -> Rint {
        vctr_len(&self.0)
    }

    fn show(&self) -> Strings {
        vctr_show(&self.0)
    }

    fn subset(self, idx: Integers) -> Self {
        let new_inner = vctr_subset(self.0, idx);
        VecUsize(new_inner)
    }

    fn extend(self, y: Self) -> Self {
        let inner = vctr_extend(self.0, y.0);
        VecUsize(inner)
    }

    fn class() -> &'static str {
        "vec_usize"
    }
}

extendr_module! {
    mod vctr;

}
