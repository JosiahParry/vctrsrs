use extendr_api::prelude::*;

mod helpers;
use crate::helpers::*;
// This is the trait that will implement structs as R vectors
// The minimum that is needed is to capture output as strings for printing
// determine the length of the vector
// subset it
// also important to be able to instantiate a new one.
pub trait Vctr {
    fn show(&self) -> Strings;
    fn length(&self) -> Rint ;
    fn subset(self, idx: Integers) -> Self;
    //fn new() -> Self;
}


// define new struct
#[derive(Debug, Clone)]
struct VecUsize(pub Vec<Option<usize>>);

// implement Vctr trait
impl Vctr for VecUsize {
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
}

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
}


extendr_module! {
    mod vctrsrs;
    fn tst_show;
    fn tst_len;
    fn tst_subset;
    fn tst_trait;
    impl VecUsize;
    fn test_mask;
}


#[extendr]
fn tst_show() -> Strings {
    struct MyTupleStruct<T>(Vec<Option<T>>);
    let v = MyTupleStruct(vec![Some(1), None, Some(2), None, Some(3)]);
    vctr_show(&v.0)
}

#[extendr]
fn tst_len() -> Rint {
    struct MyTupleStruct<T>(Vec<Option<T>>);
    let v = MyTupleStruct(vec![Some(1), None, Some(2), None, Some(3)]);
    vctr_len(&v.0)
}

#[extendr]
fn tst_subset() -> () {
    struct MyTupleStruct<T>(Vec<Option<T>>);
    let v = MyTupleStruct(vec![Some(1), None, Some(2), None, Some(3)]);

    let idx = vec![Rint::from(1), Rint::na(), Rint::from(3)].into_iter().collect::<Integers>();

    let mask = vctr_subset(v.0, idx);
    rprintln!("{:?}", mask);

}

#[extendr]
fn tst_trait() {
    let n = Integers::from_values(vec![10, 0, 11, 99, -10].iter());
    let vu = VecUsize::new(n);
    rprintln!("{:?}", vu.show());
    rprintln!("{:?}", vu.length());
    rprintln!("{:?}", vu.0);
    let id = Integers::from_values(vec![3].iter());
    rprintln!("{:?}", id);

    rprintln!("{:?}", vu.subset(id));
}

// Trying to figure out the mask :/

#[extendr]
fn test_mask() {
    let idx = vec![Some(1), Some(4), None, Some(5)];
    let n = 5 as usize;
    rprintln!("{:?}", create_mask(idx, n));
}

fn create_mask(idx: Vec<Option<i32>>, n: usize) -> Vec<Option<bool>> {
    let mut mask = vec![Some(false); n]; // initialize mask vector with None values

    for i in idx {
        if let Some(x) = i {
            let x = x - 1;
            if x >= 0 && x < n as i32 {
                mask[x as usize] = Some(true); // set the corresponding value to Some(true)
            } else if x >= n as i32 {
                let new_len = x as usize + 1; // calculate new length of mask vector
                mask.resize_with(new_len, || None); // expand mask vector with None values
                mask[x as usize] = Some(true); // set the corresponding value to Some(true)
            }
        } else {
            mask.push(None); // expand mask vector with None value
        }
    }

    mask // return the mask vector
}
    // if i == current id iter, store Some(true), id.next()
    // if i != current id iter, store Some(false)
    // if i is None, store None, id.next()
    // params
    //idx: Vec<i32>, n: usize
    // let idx = vec![Some(1), Some(4), None, Some(5)];
    // let n = 5 as usize;

    // let res = vec![true, false, false, true, true];

        // let res = vec![Some(false); n];
