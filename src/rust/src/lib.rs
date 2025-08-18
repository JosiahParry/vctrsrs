use extendr_api::prelude::*;

mod point;
use point::vctr_points;
mod vec_usize;
use vec_usize::vctr_vec_usize;

mod foo;

extendr_module! {
    mod vctrsrs;
    use foo;
    use vec_usize;
    use point;
    use vctr_points;
    use vctr_vec_usize;
}

