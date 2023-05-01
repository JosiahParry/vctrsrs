use extendr_api::prelude::*;

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

