use extendr_api::prelude::*;
use crate::rvctr::Rvctr;

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


impl<T: Rvctr> Vctr<T>  {
    pub fn as_vctr(&self) -> Robj {
        let mut x = self.inner.clone();
        x.set_class([T::class(), "vctrsrs", "vctrs_vctr"])
            .expect("failed to extract inner integers");
        x.clone().into_robj()
    }

    pub fn try_into_inner(&self) -> Result<ExternalPtr<T>> {
        // Extract the "extendr_ptr" attribute
        let ptr_attrib = self.inner.get_attrib("extendr_ptr")
            .ok_or_else(|| extendr_api::Error::ExpectedExternalPtr(().into()))?;

        // Convert the attribute into an external pointer
        let external_ptr = ExternalPtr::<T>::try_from(&ptr_attrib)?;
        Ok(external_ptr)
    }

    pub fn show(&self) -> Result<Strings> {
        Ok(self.try_into_inner()?.show())
    }
}

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


/// Implements conversion from a Vctr container to `T`
macro_rules! impl_try_from_vctr {
    ($struct:ty) => {
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
pub(crate) use impl_try_from_vctr;