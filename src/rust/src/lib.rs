mod helpers;
use crate::helpers::*;
use extendr_api::prelude::*;
mod vctr;
use vctr::*;
// mod altreptst;
// mod helpers;

// This is the trait that will implement structs as R vectors
// The minimum that is needed is to capture output as strings for printing
// determine the length of the vector
// subset it
// also important to be able to instantiate a new one.

/// Container struct for `T`
/// This will be the way to get data in and out of R
#[derive(Debug, Clone)]
pub struct Vctr<T: Rvctr> {
    pub(crate) inner: Integers,
    phantom: std::marker::PhantomData<T>,
}

/// Convert from an Robj to a Vctr container
impl<T: Rvctr> TryFrom<Robj> for Vctr<T> {
    type Error = extendr_api::Error;

    fn try_from(value: Robj) -> Result<Self> {
        let inner = Integers::try_from(value)?;

        // Check that the point is an external pointer
        let ptr = match inner.get_attrib("extendr_ptr") {
            Some(ptr) => ptr,
            None => return Err(Error::ExpectedExternalPtr(().into())),
        };

        // Here we try to convert to the external pointer
        // if this fails it is the wrong type
        let _ = ExternalPtr::<T>::try_from(&ptr)?;

        // craft the vector from the integer
        let res = Vctr {
            inner,
            phantom: std::marker::PhantomData,
        };

        Ok(res)
    }
}

/// Implements conversion from a Vctr container to `T`
macro_rules! impl_try_from_vctr {
    ($struct:ty) => {
        impl<T: Rvctr> TryFrom<Vctr<T>> for $struct {
            type Error = extendr_api::Error;

            fn try_from(value: Vctr<T>) -> Result<Self> {
                let pntr = match value.inner.get_attrib("extendr_ptr") {
                    Some(p) => p,
                    None => return Err(Self::Error::ExpectedExternalPtr(().into())),
                };
                let res = <Self>::try_from(pntr)?;
                Ok(res)
            }
        }

        impl TryFrom<Robj> for $struct
        where
            $struct: Rvctr,
        {
            type Error = extendr_api::Error;

            fn try_from(value: Robj) -> Result<Self> {
                let inner = Integers::try_from(value)?;
                let pntr = match inner.get_attrib("extendr_ptr") {
                    Some(p) => p,
                    None => return Err(Error::ExpectedExternalPtr(().into())),
                };

                // try and get the data from the pointer
                let extptr = ExternalPtr::<Self>::try_from(pntr)?;
                // make it owned
                let dat = extptr.as_ref().clone();
                Ok(dat)
            }
        }
    };
}

impl_try_from_vctr!(VecUsize);

/// Convert from `T` to a Vctr container
impl<T: Rvctr> From<T> for Vctr<T> {
    fn from(value: T) -> Self {
        let n = value.length();
        let ptr = ExternalPtr::new(value);
        let mut inner = Integers::new(n.inner() as usize);
        let inner = inner.set_attrib("extendr_ptr", ptr).unwrap().clone();
        Vctr {
            inner,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Rvctr> Vctr<T> {
    pub fn as_vctr(&self) -> Robj {
        let mut x = self.inner.clone();

        x.set_class([T::class(), "vctrsrs", "vctrs_vctr"])
            .expect("failed to extract inner integers");
        x.clone().into_robj()
    }
}

// define new struct
#[derive(Debug, Clone)]
pub struct VecUsize(pub Vec<Option<usize>>);

#[extendr]
/// @export
pub fn new_usize_vec(x: Integers) -> Robj {
    let dat = VecUsize::new(x);
    let vctr = Vctr::from(dat);
    vctr.as_vctr()
}

#[extendr]
/// @export
pub fn from_vec_usize(x: VecUsize) -> Robj {
    rprintln!("{x:#?}");
    Vctr::try_from(x).unwrap().as_vctr()
}

// TODO this needs to be turned into format.Type
/// @export
#[extendr(r_name = "format.vec_usize")]
pub fn show_vctrsrs(x: VecUsize) -> Strings {
    x.show()
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
    fn show_vctrsrs;
    // impl VecUsize;
    // use vctr;
    // use altreptst;
}
