use extendr_api::prelude::*;

mod vec_usize;
use vec_usize::vctr_vec_usize;

extendr_module! {
    mod vctrsrs;
    use vec_usize;
    use vctr_vec_usize;
}

