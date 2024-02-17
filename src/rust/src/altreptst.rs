use extendr_api::prelude::*; 


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
}