use extendr_api::prelude::*;

// Helper functions for creating your own vectors
// Takes a Vec<Option<T>> and captures the debug output as a character vec
pub fn vctr_show<T: std::fmt::Debug, S: AsRef<[Option<T>]>>(x: S) -> Strings {
    x.as_ref()
        .iter()
        .map(|xi| match xi {
            Some(xi) => format!("{:?}", xi),
            None => String::from("NA"),
        })
        .collect::<Strings>()
}

// Returns an integer of the length of the vector 
pub fn vctr_len<T: std::fmt::Debug, S: AsRef<[Option<T>]>>(x: S) -> Rint {
    Rint::from(x.as_ref().len() as i32)
}

// The trickiest one here! 
// Takes a Vec<Option<T>> and integers to subset and return a new Vec<Option<T>>
pub fn vctr_subset<T: std::fmt::Debug>(x: Vec<Option<T>>, idx: Integers) -> Vec<Option<T>> {
    // identify how many elements there are 
    let x_len = x.len();
    
    // TODO this masking is entirely incorrect
    // create an empty mask of Option<bool>
    let mut mask = vec![Some(false); x_len];

    // if any elements are 
    for (idx, i) in idx.iter().enumerate() {
        let ii = match &i {
            _ if i.is_na() => {//rprintln!("{:?}", i); 
            None },
            _ if i.inner() <= 0 || i.inner() as usize > x_len => {rprintln!("{:?}", i); None },
            _ => Some(true)
        };

        mask[idx] = ii;
    }

    // The mask 
    // Some(true) => return the Option<T>
    // None => placeholder for NA return Option<None> (is that a thing?)
    // Some(fale) => skip it
    let res: Vec<_> = x.into_iter().zip(mask.into_iter())
        .filter(|(_, i)| {
            match i {
                Some(i) => *i,
                None => true 
            }
        })
        .map(|(xi, i)| {
            match i {
                Some(_) => xi,
                None => None
            }
        }).collect();

    res
}