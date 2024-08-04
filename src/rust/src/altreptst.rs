use extendr_api::prelude::*; 
mod

#[derive(Debug, Clone)]
struct Container(Vec<Person>);

#[extendr]
impl Container {
}

#[derive(Debug, Clone)]
struct Person {
    first: String,
    last: String
}

impl AltrepImpl for Container {
    fn length(&self) -> usize {
        self.0.len()
    }
}

impl AltListImpl for Container {
    fn elt(&self, index: usize) -> Robj {
        //format!("{}", index).into()
        let inner = self.0.clone();
        
        let new_inner = inner
        .into_iter()
        .nth(index)
        .unwrap();

        Container(vec![new_inner]).into()
    }
}

#[extendr]
fn tst_altrep() -> Altrep {
    let doejane = Person{
        first: String::from("Doe"), 
        last: String::from("Jane") 
    };
    let mystate = Container(vec![doejane.clone(), doejane]);
    let class = Altrep::make_altlist_class::<Container>("ctn", "vctrsrs");
    let obj = Altrep::from_state_and_class(mystate, class, false);
    obj
}


// use std::result::Result;
use extendr_api::SEXP;
use libR_sys::{R_ExternalPtrAddr, R_ExternalPtrTag};
#[extendr]
fn frm_altrep(x: Altrep) -> () {
    // let (d1, d2) = x.data();

    // use crate::altreptst::Container; 
    let (d1, d2) = x.data();

    let d1 = d2;
    let d1_sexp  =  unsafe { d1.get() }; 
    let d1_ptr = unsafe { R_ExternalPtrAddr(d1_sexp) };
    let d1_tag = unsafe { R_ExternalPtrTag(d1_ptr) };
    unsafe { libR_sys::Rf_PrintValue(d1_tag) };
    
    // let d1: Result<ExternalPtr<Container>> = ExternalPtr::try_from(d1);
    // let d2: Result<ExternalPtr<Container>> = ExternalPtr::try_from(d2);
    // rprintln!("{:?}\n{:?}", d1, d2);

    // let addr = unsafe {d1.get() as usize};

    // let ptr = addr as *const Container; // replace YourStruct with your actual struct type
    // let your_struct: &Container = unsafe { &*ptr };

}

fn mat(x: Robj) {
    match x.rtype() {
        Rtype::Logicals => todo!(),
        Rtype::Integers => todo!(),
        Rtype::Doubles => todo!(),
        Rtype::Complexes => todo!(),
        _ => unimplemented!()
    }
}

#[extendr]
fn tst_altrepn(n: i32) -> Altrep {
    let n = n as usize; 
    let mut res_ppl = Vec::with_capacity(n as usize);
    for _ in 0..n {
       let pers = Person{
        first: String::from("Doe"), 
        last: String::from("Jane") 
       };

       res_ppl.push(pers);
    }

    let mystate = Container(res_ppl);
    let class = Altrep::make_altlist_class::<Container>("ctn", "vctrsrs");
    let obj = Altrep::from_state_and_class(mystate, class, false);
    obj
}


#[derive(Debug, Clone)]
struct StringInts {
    len: usize
}

#[extendr]
impl StringInts {

}

impl AltrepImpl for StringInts {
    fn length(&self) -> usize {
        self.len as usize
    }
}

impl AltStringImpl for StringInts {
    fn elt(&self, index: usize) -> Rstr {
        format!("{}", index).into()
    }
}



#[extendr]
fn tst_altstring() -> Altrep {
    let mystate = StringInts { len: 10 };

    let class = Altrep::make_altstring_class::<StringInts>("si", "mypkg");
    Altrep::from_state_and_class(mystate, class, false)
}


#[extendr]
fn new_stringint() -> StringInts {
    StringInts { len: 100 }
}


extendr_module! {
    mod altreptst;
    fn tst_altrep;
    fn tst_altstring;
    fn tst_altrepn;
    fn new_stringint;
    fn frm_altrep;
}