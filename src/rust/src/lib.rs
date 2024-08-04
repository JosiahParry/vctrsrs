mod helpers;
use crate::helpers::*;
use extendr_api::{prelude::*, ToVectorValue};
mod vctr;
use vctr::*;
// mod altreptst;
// mod helpers;

// This is the trait that will implement structs as R vectors
// The minimum that is needed is to capture output as strings for printing
// determine the length of the vector
// subset it
// also important to be able to instantiate a new one.

#[derive(Debug, Clone)]
pub struct VctrContainer(Integers);

impl IntoRobj for VctrContainer {
    fn into_robj(self) -> Robj {
        self.0.into_robj()
    }
}

impl TryFrom<Robj> for VctrContainer {
    type Error = extendr_api::Error;

    fn try_from(value: Robj) -> Result<Self> {
        let inner = Integers::try_from(value)?;

        // Check that the point is an external pointer
        match inner.get_attrib("extendr_ptr") {
            Some(_) => (),
            None => return Err(Error::ExpectedExternalPtr(().into())),
        }

        // create the container with the integer vector
        Ok(VctrContainer(inner))
    }
}

// define new struct
#[derive(Debug, Clone)]
pub struct VecUsize(pub Vec<Option<usize>>);

#[extendr]
pub fn new_usize_vec(x: Integers) -> Integers {
    let dat = VecUsize::new(x);
    let n = dat.0.len();
    let mut res = Integers::new(n);

    res.set_attrib("extendr_ptr", ExternalPtr::new(dat))
        .unwrap()
        .clone()
}

#[extendr]
pub fn from_vec_usize(x: VecUsize) -> Robj {
    rprintln!("{x:#?}");
    VctrContainer::try_from(x).unwrap().into_robj()
}

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

    pub fn length(&self) -> Rint {
        vctr_len(&self.0)
    }

    pub fn show(&self) -> Strings {
        vctr_show(&self.0)
    }

    // Clone is used here because of the way that subsetting in R happend
    // the same element can be grabbed multiple times which would
    pub fn subset(&self, idx: Integers) -> Self {
        let inner = self.0.clone();
        let new_inner = vctr_subset(inner, idx);
        VecUsize(new_inner)
    }
}

extendr_module! {
    mod vctrsrs;
    fn new_usize_vec;
    fn from_vec_usize;
    // impl VecUsize;
    // use vctr;
    // use altreptst;
}
