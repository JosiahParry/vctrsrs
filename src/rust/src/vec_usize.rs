use extendr_api::prelude::*;
use extendr_vctrs::{extendr_vctr, Rvctr, Vctr};


#[derive(Clone, Debug, Rvctr)]
#[extendr_vctr("vec_usize")]
pub struct VecUsize(pub Vec<Option<usize>>);


/// @export
#[extendr]
pub fn new_usize_vec(x: Integers) -> VecUsize {
    VecUsize::new(x)
}



// The `Vctr` Struct is intended to be a simple wrapper that
// contains any custom types that is defined by the developer.
// The Vctr struct requires that any object contained by it must
// implement the Rvctr trait which is minimal
// //implement Vctr trait
// add extendr implementation with new method
impl VecUsize {
    pub fn new(robj: Integers) -> Self {
        let x = robj
            .iter()
            .map(|x| match &x {
                _ if x.is_na() => None,
                _ if x.inner() < 0 => None,
                _ => Some(x.inner() as usize),
            })
            .collect();
        VecUsize(x)
    }
}




extendr_module! {
    mod vec_usize;
    fn new_usize_vec;
}