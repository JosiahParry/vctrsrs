use crate::helpers::*;
use crate::VctrContainer;
use crate::VecUsize;
use extendr_api::prelude::*;

// This is the trait that all Rust objects that want to
// be treated as R vectors will need to implement
pub trait Rvctr {
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
}

// Implement TryFrom VctrContainer for all structs that implement Rvctr
impl TryFrom<VctrContainer> for VecUsize {
    type Error = extendr_api::Error;

    fn try_from(value: VctrContainer) -> Result<Self> {
        let pntr = match value.0.get_attrib("extendr_ptr") {
            Some(p) => p,
            None => return Err(Error::ExpectedExternalPtr(().into())),
        };
        let res = <Self>::try_from(pntr)?;
        Ok(res)
    }
}

impl TryFrom<VecUsize> for VctrContainer {
    type Error = extendr_api::Error;

    fn try_from(value: VecUsize) -> Result<Self> {
        let n = value.0.len();
        let mut res = Integers::new(n);
        res.set_attrib("extendr_ptr", ExternalPtr::new(value))
            .unwrap();
        Ok(VctrContainer(res))
    }
}

impl TryFrom<Robj> for VecUsize {
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

// this could be created by a macro...
// #[extendr]
// impl Vctr<VecUsize> {
//     fn length(&self) -> Rint {
//         self.0.length()
//     }

//     fn show(&self) -> Strings {
//         self.0.show()
//     }

//     fn subset(&mut self, idx: Integers) -> Self {
//         let x = self.0.clone();
//         let res = x.subset(idx);
//         Vctr(res)
//     }
// }

// #[extendr]
// fn tst_vctr_usize() -> Vctr<VecUsize> {
//     let id = Integers::from_values(vec![3, 1, 1000, 99, 33].iter());
//     Vctr(VecUsize::new(id))
// }

// This function illustrates that we can provide subclasses
// to impl objects
// I would love to have something like

// #[extendr(class = "usize")]
// struct VecUsize(Vec<Option<T>>);
// This would tell extendr that anytime that this class is encountered
// the class could be applied to it as well.

// For example I would love to have
// #[extendr(class = "usize")]
// impl Vctr<VecUsize> {
// }

// #[extendr]
// fn tst_vctr_class() -> Robj {
//     let x = tst_vctr_usize();
//     x.into_robj()
//         .set_class(&["usize", "Vctr"])
//         .unwrap().clone
// }

extendr_module! {
    mod vctr;
    // impl Vctr<VecUsize>;
    // fn tst_vctr_usize;
    // fn tst_vctr_class;
    // fn new_usize;
}

// This is the Trait object approach which I do not think will work
// pub struct Vctrs(pub Box<dyn Rvctr>);
// #[extendr]
// impl Vctrs {
//     fn show(&self) -> Strings {
//         self.0.show()
//     }

//     fn length(&self) -> Rint {
//         self.0.length()
//     }

//     fn subset(&self, idx: Integers) -> Vctrs {
//         self.0.clone().subset(idx)
//     }
// }
