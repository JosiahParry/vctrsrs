use extendr_api::prelude::*;

mod vctr;
mod helpers;

// This is the trait that will implement structs as R vectors
// The minimum that is needed is to capture output as strings for printing
// determine the length of the vector
// subset it
// also important to be able to instantiate a new one.



// define new struct
#[derive(Debug, Clone)]
pub struct VecUsize(pub Vec<Option<usize>>);


// add extendr implementation with new method
#[extendr]
impl VecUsize {
    pub fn new(robj: Integers) -> Self {
        let x = robj
            .iter()
            .map(|x| match &x {
                _ if x.is_na() => None,
                _ if x.inner() < 0 => None,
                _ => Some(x.0 as usize),
            })
            .collect();
        VecUsize(x)
    }

    // pub fn length(&self) -> Rint {
    //     vctr_len(&self.0)
    // }

    // pub fn show(&self) -> Strings {
    //     vctr_show(&self.0)
    // }

    // // Clone is used here because of the way that subsetting in R happend
    // // the same element can be grabbed multiple times which would
    // pub fn subset(&self, idx: Integers) -> Self {
    //     let inner = self.0.clone();
    //     let new_inner = vctr_subset(inner, idx);
    //     VecUsize(new_inner)
    // }
}


extendr_module! {
    mod vctrsrs;
    impl VecUsize;
    use vctr;
}
