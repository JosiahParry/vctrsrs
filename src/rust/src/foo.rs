use extendr_api::prelude::*;

pub struct VecUsize(pub Vec<usize>);
#[extendr]
impl VecUsize {}


pub mod test {
    use extendr_api::prelude::*;

    #[extendr]
    pub fn hello_world() -> String {
        String::from("Hello, world!")
    }

    pub struct Point {
        x: f64,
          y: f64
    }

    #[extendr]
    impl Point {}

    extendr_module! {
        mod test;
        fn hello_world;
        impl Point;
    }
}

extendr_module! {
    mod foo;
    use test;
    impl VecUsize;
}
