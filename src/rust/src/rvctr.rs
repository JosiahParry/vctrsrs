
use extendr_api::prelude::*;

// This is the trait that all Rust objects that want to
// be treated as R vectors will need to implement
pub trait Rvctr where 
    Self: Sized 
{
    fn class() -> &'static str;
    fn show(&self) -> Strings;
    fn length(&self) -> Rint;
    fn subset(self, idx: Integers) -> Self;
    fn extend(self, y: Self) -> Self;
}

