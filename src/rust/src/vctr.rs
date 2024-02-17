use extendr_api::prelude::*;
use crate::VecUsize;
use crate::helpers::*;

// This is the trait that all Rust objects that want to 
// be treated as R vectors will need to implement
pub trait Rvctr {
    fn show(&self) -> Strings;
    fn length(&self) -> Rint ;
    fn subset(self, idx: Integers) -> Self;
    fn extend(self, y: Self) -> Self;
}

// The `Vctr` Struct is intended to be a simple wrapper that 
// contains any custom types that is defined by the developer. 
// The Vctr struct requires that any object contained by it must
// implement the Rvctr trait which is minimal
pub struct Vctr<T: Rvctr>(pub T);

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


// this could be created by a macro...
#[extendr]
impl Vctr<VecUsize> {
    fn length(&self) -> Rint {
        self.0.length()
    }

    fn show(&self) -> Strings {
        self.0.show()
    }

    fn subset(&mut self, idx: Integers) -> Self {
        let x = self.0.clone();
        let res = x.subset(idx);
        Vctr(res)
    }
}


#[extendr]
fn tst_vctr_usize() -> Vctr<VecUsize> {
    let id = Integers::from_values(vec![3, 1, 1000, 99, 33].iter());
    Vctr(VecUsize::new(id))
}

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

#[extendr]
fn tst_vctr_class() -> Robj {
    let x = tst_vctr_usize();
    x.into_robj()
        .set_class(&["usize", "Vctr"])
        .unwrap()
}

#[extendr]
/// @export
pub fn new_usize(x: Integers) -> Vctr<VecUsize> {
    Vctr(VecUsize::new(x))
}


extendr_module! {
    mod vctr;
    impl Vctr<VecUsize>;
    fn tst_vctr_usize;
    fn tst_vctr_class;
    fn new_usize;
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
